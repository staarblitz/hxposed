#![allow(dead_code)]

use crate::win::{ExAllocatePool2, PoolFlags};
use alloc::boxed::Box;

pub trait PoolAlloc<T> {
    fn alloc() -> Box<T>;
    fn alloc_sized(size: usize) -> Box<T>;
}
impl<T> PoolAlloc<T> for T {
    fn alloc() -> Box<T> {
        let alloc = unsafe { ExAllocatePool2(PoolFlags::NonPaged, size_of::<T>() as _, 0x2009) };
        unsafe { Box::from_raw(alloc as _) }
    }
    fn alloc_sized(size: usize) -> Box<T> {
        let alloc = unsafe { ExAllocatePool2(PoolFlags::NonPaged, size as _, 0x2009) };
        unsafe { Box::from_raw(alloc as _) }
    }
}
