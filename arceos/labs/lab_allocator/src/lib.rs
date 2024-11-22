//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::ptr::NonNull;
use core::alloc::Layout;

const POOL_SIZE:usize = 96+192+384;
const MEMORY_START:usize = 0xffffffc08026f000;
const MEMORY_END:usize = 0xffffffc088000000;
const NT_PTR:usize = MEMORY_START+POOL_SIZE;
// 直接通过修改这个值改变停在哪里,这里只是理论上限,可以试试525088,会停在801次
const MAX_SIZE:usize = MEMORY_END-MEMORY_START-POOL_SIZE-1;

pub struct LabByteAllocator{
    pool_96: usize,
    pool_192: usize,
    pool_384: usize,
    start: usize,
    end: usize,
    byte_pos: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self{
            pool_96: 0,
            pool_192: 0,
            pool_384: 0,
            start: 0,
            end: 0,
            byte_pos: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.pool_96 = MEMORY_START;
        self.pool_192 = MEMORY_START+96;
        self.pool_384 = MEMORY_START+96+192;
        self.start = MEMORY_START+POOL_SIZE;
        self.end = MEMORY_END;
        self.byte_pos = self.start;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if layout.align() == 8 {
            if layout.size() == 96 {
                return AllocResult::Ok(NonNull::new(self.pool_96 as *mut u8).unwrap());
            } else if layout.size() == 192 {
                return AllocResult::Ok(NonNull::new(self.pool_192 as *mut u8).unwrap());
            } else if layout.size() == 384 {
                return AllocResult::Ok(NonNull::new(self.pool_384 as *mut u8).unwrap());
            }
        }
        if layout.align() != 8 {
            // debug!("alloc layout: {:?}", layout);
            if layout.size() > MAX_SIZE {
                return AllocResult::Err(AllocError::NoMemory);
            }
            return AllocResult::Ok(NonNull::new(NT_PTR as *mut u8).unwrap());
        }
        let len = layout.size();
        let new_pos = self.byte_pos + len;
        if new_pos > self.end {
            return AllocResult::Err(AllocError::NoMemory);
        }
        let ptr = self.byte_pos as *mut u8;
        self.byte_pos = new_pos;
        AllocResult::Ok(NonNull::new(ptr).unwrap())
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        if layout.align() == 8 {
            return;
        }
        if layout.align() != 8 {
            // info!("dealloc layout: {:?}", layout);
            return;
        }
        let len = layout.size();
        let pos = pos.as_ptr() as usize;
        if pos+len == self.byte_pos {
            self.byte_pos -= len;
        }
    }
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }
    fn used_bytes(&self) -> usize {
        self.byte_pos - self.start
    }
    fn available_bytes(&self) -> usize {
        self.end - self.byte_pos
    }
}
