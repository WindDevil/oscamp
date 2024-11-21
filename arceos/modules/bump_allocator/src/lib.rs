#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///

pub struct EarlyAllocator<const PAGE_SIZE: usize>{
    start: usize,
    end: usize,
    byte_pos: usize,
    page_pos: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self{
        Self{
            start:0,
            end:0,
            byte_pos:0,
            page_pos:0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start+size;
        self.byte_pos = start;
        self.page_pos = self.end;
    }

    // 因为是earlyallocator,没有什么内存碎片需要重新回收,因此就不需要这个添加的结构
    // 对应的alt_allocator的
    fn add_memory(&mut self, _start: usize, _size: usize) -> allocator::AllocResult {
       unimplemented!() 
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: core::alloc::Layout) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let len = layout.size();
        if self.available_bytes() < len {
            return Err(allocator::AllocError::NoMemory);
        }
        let ptr = self.byte_pos as *mut u8;
        self.byte_pos += len;
        Ok(unsafe { core::ptr::NonNull::new_unchecked(ptr) })
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        let len = layout.size();
        let pos = pos.as_ptr() as usize;
        if pos+layout.size() == self.byte_pos {
            self.byte_pos -= len;
        }
    }

    fn available_bytes(&self) -> usize {
        return self.page_pos-self.byte_pos;
    }

    fn total_bytes(&self) -> usize {
        return self.end-self.start;
    }

    fn used_bytes(&self) -> usize {
        return self.byte_pos-self.start+self.end-self.page_pos;
    }
}

#[inline]
const fn align_down(pos: usize, align: usize) -> usize {
    pos & !(align - 1)
}

#[inline]
const fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}



impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {

    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> allocator::AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(allocator::AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(allocator::AllocError::InvalidParam);
        }
        let start = align_up(self.page_pos - num_pages * PAGE_SIZE,align_pow2);
        if start < self.byte_pos {
            return Err(allocator::AllocError::NoMemory);
        }
        self.page_pos = start;
        Ok(start)
    }

    fn available_pages(&self) -> usize {
        let end = align_down(self.page_pos, PAGE_SIZE);
        let start = align_up(self.start, PAGE_SIZE);
        (end - start) / PAGE_SIZE
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        if pos == self.page_pos {
            self.page_pos += num_pages * PAGE_SIZE;
        }
    }

    fn total_pages(&self) -> usize {
        let end = align_down(self.end, PAGE_SIZE);
        let start = align_up(self.start, PAGE_SIZE);
        (end - start) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        self.total_pages() - self.available_pages()
    }
    
}
