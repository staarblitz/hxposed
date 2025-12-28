use crate::utils::danger::DangerPtr;
use core::ptr::null_mut;
use wdk_sys::_MEMORY_CACHING_TYPE::{MmCached, MmNonCached};
use wdk_sys::_MM_PAGE_PRIORITY::HighPagePriority;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{
    ExFreePool, IoAllocateMdl, MmAllocatePagesForMdl, MmAllocatePagesForMdlEx, MmFreePagesFromMdl,
    MmMapLockedPagesSpecifyCache, MmUnmapLockedPages,
};
use wdk_sys::{_MDL, FALSE, MM_ALLOCATE_PREFER_CONTIGUOUS, PHYSICAL_ADDRESS};

/// Abstraction over MDL with Rust safety.
pub struct MemoryDescriptor {
    mdl: DangerPtr<_MDL>,
    pub status: MapStatus,
    pub length: usize,
}

unsafe impl Send for MemoryDescriptor {}
unsafe impl Sync for MemoryDescriptor {}

pub enum MapStatus {
    NotInitialized,
    Mapped(usize),
    Allocated,
}

impl Drop for MemoryDescriptor {
    fn drop(&mut self) {
        unsafe {
            MmFreePagesFromMdl(self.mdl.ptr);
            ExFreePool(self.mdl.ptr as _);
        }
        if let MapStatus::Mapped(location) = self.status {
            unsafe { MmUnmapLockedPages(location as _, self.mdl.ptr) }
        }
    }
}

impl MemoryDescriptor {
    pub const fn default() -> Self {
        Self {
            mdl: DangerPtr {
                ptr: null_mut()
            },
            length: 0,
            status: MapStatus::NotInitialized,
        }
    }

    pub fn new(length: usize) -> Self {
        let mut me = Self::default();
        Self::init(&mut me, length);
        me
    }

    pub fn init(&mut self, length: usize) {
        const ZERO: PHYSICAL_ADDRESS = PHYSICAL_ADDRESS { QuadPart: 0 };
        const MAX: PHYSICAL_ADDRESS = PHYSICAL_ADDRESS { QuadPart: i64::MAX };

        let mdl = DangerPtr {
            ptr: unsafe {
                MmAllocatePagesForMdlEx(
                    ZERO,
                    MAX,
                    ZERO,
                    length as _,
                    MmNonCached,
                    MM_ALLOCATE_PREFER_CONTIGUOUS,
                )
            },
        };

        self.status = MapStatus::Allocated;
        self.length = length;
        self.mdl = mdl;
    }

    pub fn map(&mut self, address: Option<usize>) {
        let address = unsafe {
            MmMapLockedPagesSpecifyCache(
                self.mdl.ptr,
                KernelMode as _,
                MmCached,
                address.unwrap_or(0) as _,
                FALSE as _,
                HighPagePriority as _,
            )
        };

        self.status = MapStatus::Mapped(address as _);
    }
}
