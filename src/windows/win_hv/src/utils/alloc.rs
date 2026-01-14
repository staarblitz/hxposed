#![allow(dead_code)]

use alloc::boxed::Box;
use wdk_sys::POOL_FLAG_NON_PAGED;
use wdk_sys::ntddk::ExAllocatePool2;

pub trait PoolAlloc<T> {
    fn alloc() -> Box<T>;
}

pub trait PoolAllocSized<T> {
    fn alloc_sized(size: usize) -> Box<T>;
}

impl<T> PoolAllocSized<T> for T {
    fn alloc_sized(size: usize) -> Box<T> {
        let alloc = unsafe { ExAllocatePool2(POOL_FLAG_NON_PAGED, size as _, 0x2009) };
        unsafe { Box::from_raw(alloc as _) }
    }
}

impl<T> PoolAlloc<T> for T {
    fn alloc() -> Box<T> {
        let alloc = unsafe { ExAllocatePool2(POOL_FLAG_NON_PAGED, size_of::<T>() as _, 0x2009) };
        unsafe { Box::from_raw(alloc as _) }
    }
}
