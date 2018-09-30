#![no_std]

mod alloc_sys;

use self::alloc_sys::*;
use core::alloc::{GlobalAlloc, Layout};

const KMRS_TAG: u32 = 0x4B4D5253; // 'KMRS'

pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ExAllocatePoolWithTag(POOL_TYPE::PagedPool, layout.size(), KMRS_TAG)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePoolWithTag(ptr, KMRS_TAG);
    }
}
