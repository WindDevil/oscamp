//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::ptr::NonNull;
use core::alloc::Layout;

const POOL_SIZE:usize = 0x40000;
const MEMORY_START:usize = 0xffff_ffc0_0000_0000; // 这里注意是一个虚拟地址
const MEMORY_END:usize = 0xffff_ffc0_8000_0000; // 这里注意是一个虚拟地址
const EVEN_END:usize = MEMORY_END - 1;
const EVEN_SIZE:usize = EVEN_END - MEMORY_START - POOL_SIZE;

pub struct LabByteAllocator{
    pool_start: usize,
    pool_end: usize,
    even_end: usize,
    start: usize,
    end: usize,
    byte_pos: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self{
            pool_start: 0,
            pool_end: 0,
            even_end: 0,
            start: 0,
            end: 0,
            byte_pos: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.pool_start = MEMORY_START;
        self.pool_end = MEMORY_START+POOL_SIZE;
        self.even_end = MEMORY_START+POOL_SIZE + EVEN_SIZE;
        self.end = MEMORY_END;
        self.start = self.pool_end;
        self.byte_pos = self.pool_end;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
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
