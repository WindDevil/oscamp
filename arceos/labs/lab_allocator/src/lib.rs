//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::ptr::NonNull;
use core::alloc::Layout;

pub struct LabByteAllocator{
    start: usize,
    end: usize,
    byte_pos: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self{
            start: 0,
            end: 0,
            byte_pos: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.byte_pos = start;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.start = start;
        self.end = start + size;
        self.byte_pos = start;
        AllocResult::Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let len = layout.size();
        let new_pos = self.byte_pos + len;
        if new_pos > self.end {
            return AllocResult::Err(AllocError::NoMemory);
        }
        AllocResult::Ok(NonNull::new( as *mut u8).unwrap())
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unimplemented!();
    }
    fn total_bytes(&self) -> usize {
        unimplemented!();
    }
    fn used_bytes(&self) -> usize {
        unimplemented!();
    }
    fn available_bytes(&self) -> usize {
        unimplemented!();
    }
}
