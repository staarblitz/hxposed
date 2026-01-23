use crate::utils::alloc::PoolAllocSized;
use crate::utils::danger::DangerPtr;
use core::hash::{Hash, Hasher};
use core::ptr::null_mut;
use wdk_sys::_MEMORY_CACHING_TYPE::{MmCached, MmNonCached};
use wdk_sys::_MM_PAGE_PRIORITY::HighPagePriority;
use wdk_sys::ntddk::{
    ExAllocatePool2, ExFreePool, IoAllocateMdl, MmAllocatePagesForMdlEx, MmBuildMdlForNonPagedPool, MmFreePagesFromMdl, MmMapLockedPagesSpecifyCache, MmProtectMdlSystemAddress, MmUnmapLockedPages
};
use wdk_sys::{
    _MDL, FALSE, KPROCESSOR_MODE, MM_ALLOCATE_PREFER_CONTIGUOUS, NTSTATUS, PFN_NUMBER, PHYSICAL_ADDRESS, PIRP, POOL_FLAG_NON_PAGED, PVOID, STATUS_ACCESS_VIOLATION, STATUS_MEMORY_NOT_ALLOCATED, STATUS_SUCCESS
};

/// Abstraction over MDL with Rust safety.
#[derive(Debug)]
pub struct MemoryDescriptor {
    pub mdl: DangerPtr<_MDL>,
    pub status: MapStatus,
    pub length: usize,
    pub owns: bool,
    pub init: bool,
}

impl Hash for MemoryDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.mdl.ptr as _);
    }
}

unsafe impl Send for MemoryDescriptor {}
unsafe impl Sync for MemoryDescriptor {}

#[derive(Debug)]
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
            mdl: DangerPtr { ptr: null_mut() },
            length: 0,
            status: MapStatus::NotInitialized,
            owns: false,
            init:false,
        }
    }

    pub fn from_raw(ptr: PVOID, length: usize) -> Self {
        let mut me = Self::default();
        let mdl = unsafe {
            ExAllocatePool2(POOL_FLAG_NON_PAGED, (size_of::<_MDL>() + size_of::<PFN_NUMBER>() * (((ptr as usize) + length + 4095) >> 12)) as u64, 0x2009)
        };
        me.mdl = DangerPtr { ptr: mdl as _ };
        me.init = true;
        me.owns = true;
        me.length = length;
        me.status = MapStatus::Allocated;

        me
    }

    pub fn new_describe(ptr: PVOID, length: u32) -> Self {
        let me = Self {
            mdl: DangerPtr {
                ptr: unsafe { IoAllocateMdl(ptr, length, FALSE as _, FALSE as _, PIRP::default()) },
            },
            length: length as _,
            status: MapStatus::Allocated,
            owns: true,
            init:true
        };
        // this is crucial. because we should let the mdl know it's for non paged pool after IoAllocateMdl when it's going to be mapped to a user process.
        unsafe {
            MmBuildMdlForNonPagedPool(me.mdl.ptr);
        }

        me
    }

    pub fn new(length: usize) -> Self {
        let mut me = Self::default();
        Self::init(&mut me, length);
        me
    }

    pub fn init(&mut self, length: usize) {
        const ZERO: PHYSICAL_ADDRESS = PHYSICAL_ADDRESS { QuadPart: 0 };
        const MAX: PHYSICAL_ADDRESS = PHYSICAL_ADDRESS { QuadPart: i64::MAX };

        self.status = MapStatus::Allocated;
        self.length = length;
        self.mdl = DangerPtr {
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
        self.owns = true;
        self.init = true;
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

    pub fn get_system_address(&self) -> usize {
        self.mdl.MappedSystemVa as _
    }

    pub fn protect(
        &self,
        protection: u32
    ) -> Result<(), NTSTATUS> {
        match unsafe {
            MmProtectMdlSystemAddress(self.mdl.ptr, protection)
        } {
            STATUS_SUCCESS => Ok(()),
            err => Err(err)
        }
    }

    pub fn map(
        &mut self,
        address: Option<usize>,
        mode: KPROCESSOR_MODE,
        flags: i32,
    ) -> Result<usize, NTSTATUS> {
        let address = match microseh::try_seh(|| unsafe {
            MmMapLockedPagesSpecifyCache(
                self.mdl.ptr,
                mode,
                MmCached,
                address.unwrap_or(0) as _,
                FALSE as _,
                flags as _,
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
