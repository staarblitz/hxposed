use crate::utils::danger::DangerPtr;
use core::ptr::null_mut;
use wdk_sys::ntddk::{ExFreePool, IoAllocateMdl, MmAllocatePagesForMdlEx, MmBuildMdlForNonPagedPool, MmFreePagesFromMdl, MmMapLockedPagesSpecifyCache, MmUnmapLockedPages};
use wdk_sys::_MEMORY_CACHING_TYPE::{MmCached, MmNonCached};
use wdk_sys::_MM_PAGE_PRIORITY::HighPagePriority;
use wdk_sys::{FALSE, KPROCESSOR_MODE, MM_ALLOCATE_PREFER_CONTIGUOUS, NTSTATUS, PHYSICAL_ADDRESS, PIRP, PVOID, STATUS_ACCESS_VIOLATION, STATUS_MEMORY_NOT_ALLOCATED, _MDL};

/// Abstraction over MDL with Rust safety.
pub struct MemoryDescriptor {
    pub mdl: DangerPtr<_MDL>,
    pub status: MapStatus,
    pub length: usize,
    pub owns: bool,
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
        if self.owns {
            unsafe {
                MmFreePagesFromMdl(self.mdl.ptr);
                ExFreePool(self.mdl.ptr as _);
            }
            if let MapStatus::Mapped(location) = self.status {
                unsafe { MmUnmapLockedPages(location as _, self.mdl.ptr) }
            }
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
            owns: false,
        }
    }

    pub fn new_describe(ptr: PVOID, length: u32) -> Self {
        let me = Self {
            mdl: DangerPtr {
                ptr: unsafe { IoAllocateMdl(ptr, length, FALSE as _, FALSE as _, PIRP::default()) }
            },
            length: length as _,
            status: MapStatus::Allocated,
            owns: true,
        };
        // this is crucial. because we should let the mdl know it's for non paged pool after IoAllocateMdl when it's going to be mapped to a user process.
        unsafe {
            MmBuildMdlForNonPagedPool(me.mdl.ptr);
        }

        me
    }

    pub fn new(length: usize) -> Self {
        let mut me = Self::default();
        Self::init(&mut me,length);
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
        self.owns = true;
    }

    pub fn unmap(&mut self) -> Result<(), ()> {
        match self.status {
            MapStatus::Mapped(ptr) => {
                unsafe { MmUnmapLockedPages(ptr as _, self.mdl.ptr) };
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn map(&mut self, address: Option<usize>, mode: KPROCESSOR_MODE) -> Result<usize, NTSTATUS> {
        let address = match microseh::try_seh(|| unsafe {
            MmMapLockedPagesSpecifyCache(
                self.mdl.ptr,
                mode,
                MmCached,
                address.unwrap_or(0) as _,
                FALSE as _,
                HighPagePriority as _,
            )
        }) {
            Ok(ptr) => match ptr as usize {
                0 => return Err(STATUS_MEMORY_NOT_ALLOCATED),
                x => x,
            },
            Err(_) => return Err(STATUS_ACCESS_VIOLATION),
        };

        self.status = MapStatus::Mapped(address as _);
        Ok(address as _)
    }
}
