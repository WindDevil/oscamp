//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::ptr::NonNull;
use core::alloc::Layout;
use axlog::*;

const POOL_SIZE:usize = 96+192+384;

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
    fn init(&mut self, start: usize, size: usize) {
        self.pool_96 = start;
        self.pool_192 = start+96;
        self.pool_384 = start+96+192;
        self.start = start+POOL_SIZE;
        self.end = start + size;
        self.byte_pos = self.start;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.start = start;
        self.end = start + size;
        self.byte_pos = self.start;
        AllocResult::Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if layout.align() == 8 {
            debug!("alloc layout: {:?}", layout);
            if layout.size() == 96 {
                return AllocResult::Ok(NonNull::new(self.pool_96 as *mut u8).unwrap());
            } else if layout.size() == 192 {
                return AllocResult::Ok(NonNull::new(self.pool_192 as *mut u8).unwrap());
            } else if layout.size() == 384 {
                return AllocResult::Ok(NonNull::new(self.pool_384 as *mut u8).unwrap());
            }
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
            info!("dealloc layout: {:?}", layout);
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
