use crate::win::{ExAllocatePool2, ExFreePool, PoolFlags};
use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::NonNull;

pub struct WdkAllocator;

unsafe impl GlobalAlloc for WdkAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { ExAllocatePool2(PoolFlags::NonPaged, layout.size(), 0x2009) };
        ptr.cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            ExFreePool(ptr as _);
        }
    }
}
