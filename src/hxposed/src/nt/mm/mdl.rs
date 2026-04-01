use crate::utils::danger::DangerPtr;
use crate::utils::logger::LogEvent;
use crate::win::{Boolean, ExAllocatePool2, ExFreePool, IoAllocateMdl, IoFreeMdl, LockOperation, MdlFlags, MemoryCacheType, MmAllocatePagesForMdlEx, MmBuildMdlForNonPagedPool, MmFreePagesFromMdl, MmMapLockedPagesSpecifyCache, MmProbeAndLockPages, MmProtectMdlSystemAddress, MmUnlockPages, MmUnmapLockedPages, NtStatus, PagePriority, PoolFlags, ProcessorMode, MDL, PVOID};
use crate::GLOBAL_LOGGER;
use alloc::boxed::Box;
use core::hash::{Hash, Hasher};
use core::ptr::null_mut;

/// Abstraction over MDL with Rust safety.
#[derive(Debug)]
pub struct MemoryDescriptor {
    pub mdl: DangerPtr<MDL>,
    pub status: MapStatus,
    pub length: usize,
    pub owns: OwnType,
    pub init: bool,
}

impl Hash for MemoryDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.mdl.ptr as _);
    }
}

unsafe impl Send for MemoryDescriptor {}
unsafe impl Sync for MemoryDescriptor {}

#[derive(Debug, Eq, PartialEq)]
pub enum OwnType {
    NonPaged(usize),
    MmAllocatePages,
    NonOwning,
    Locked,
}

#[derive(Debug)]
pub enum MapStatus {
    NotInitialized,
    Mapped(usize),
    Allocated,
}

impl Drop for MemoryDescriptor {
    fn drop(&mut self) {
        match self.owns {
            OwnType::NonPaged(addr) => unsafe { ExFreePool(addr as _) },
            OwnType::MmAllocatePages => unsafe { MmFreePagesFromMdl(self.mdl.ptr) },
            OwnType::Locked => unsafe {
                MmUnlockPages(self.mdl.ptr);
            },
            OwnType::NonOwning => {
                // do nothing
            }
        }
        if let MapStatus::Mapped(location) = self.status {
            unsafe { MmUnmapLockedPages(location as _, self.mdl.ptr) }
        }

        // free the mdl at the end
        unsafe {
            IoFreeMdl(self.mdl.ptr as _);
        }
    }
}

impl MemoryDescriptor {
    pub const fn default() -> Self {
        Self {
            mdl: DangerPtr { ptr: null_mut() },
            length: 0,
            status: MapStatus::NotInitialized,
            owns: OwnType::NonOwning,
            init: false,
        }
    }

    pub fn from_raw(ptr: PVOID, length: usize) -> Self {
        let mut me = Self::default();
        let mdl = unsafe {
            ExAllocatePool2(
                PoolFlags::NonPaged,
                size_of::<MDL>() + 8 * (((ptr as usize) + length + 4095) >> 12),
                0x2009,
            )
        };
        me.mdl = DangerPtr { ptr: mdl as _ };
        me.init = true;
        me.owns = OwnType::NonOwning;
        me.length = length;
        me.status = MapStatus::Allocated;

        me
    }

    /// Must be in context of process to describe the pages
    pub fn lock_pages(ptr: PVOID, length: u32) -> Option<Self> {
        let mut me = Self::new_describe(ptr, length)?;
        microseh::try_seh(|| unsafe {
            MmProbeAndLockPages(
                me.mdl.ptr,
                ProcessorMode::UserMode,
                LockOperation::IoWriteAccess,
            )
        })
        .ok()
        .map(|_| {
            me.owns = OwnType::Locked;
            me
        })
    }

    pub fn new_describe(ptr: PVOID, length: u32) -> Option<Self> {
        match (Self {
            mdl: DangerPtr {
                ptr: unsafe {
                    IoAllocateMdl(ptr, length, Boolean::False, Boolean::False, null_mut())
                },
            },
            length: length as _,
            status: MapStatus::Allocated,
            // no, we do not own. we just "describe" the existing pages.
            owns: OwnType::NonOwning,
            init: true,
        }) {
            desc if desc.mdl.ptr.is_null() => None,
            desc => Some(desc),
        }
    }

    pub fn new_describe_nonpaged(ptr: PVOID, length: u32) -> Option<Self> {
        let me = Self::new_describe(ptr, length)?;
        // this is crucial. because we should let the mdl know it's for non paged pool after IoAllocateMdl when it's going to be mapped to a user process.
        unsafe {
            MmBuildMdlForNonPagedPool(me.mdl.ptr);
        }

        Some(me)
    }

    pub fn new(length: usize) -> Result<Self, NtStatus> {
        let mut me = Self::default();
        Self::init(&mut me, length).map(|_| me)
    }

    pub fn new_nonpaged(length: u32) -> Option<Self> {
        let pool = unsafe { Box::<[u8; 4096]>::new_zeroed().assume_init() };

        let mut me = Self::new_describe_nonpaged(pool.as_ref().as_ptr() as _, length as _)?;

        me.owns = OwnType::NonPaged(Box::into_raw(pool) as _);

        Some(me)
    }

    pub fn init(&mut self, length: usize) -> Result<(), NtStatus> {
        self.status = MapStatus::Allocated;
        self.length = length;
        self.mdl = DangerPtr {
            ptr: unsafe {
                MmAllocatePagesForMdlEx(
                    0,
                    i64::MAX,
                    0,
                    length as _,
                    MemoryCacheType::MmNonCached,
                    0,
                )
            },
        };

        if self.mdl.ptr.is_null() {
            return Err(NtStatus::Unsuccessful);
        }

        self.owns = OwnType::MmAllocatePages;
        self.init = true;

        Ok(())
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
    pub fn get_system_address_safe(&mut self) -> Result<PVOID, NtStatus> {
        if self.mdl.MdlFlags as u64
            & (MdlFlags::MappedToSystemVa as u64 | MdlFlags::SourceIsNonpagedPool as u64)
            == 1
        {
            Ok(self.mdl.MappedSystemVa)
        } else {
            self.map(
                None,
                ProcessorMode::KernelMode,
                PagePriority::HighPagePriority as _,
            )
        }
    }

    pub fn protect(&self, protection: u32) -> Result<(), NtStatus> {
        match unsafe { MmProtectMdlSystemAddress(self.mdl.ptr, protection) } {
            NtStatus::Success => Ok(()),
            err => Err(err),
        }
    }

    pub fn map(
        &mut self,
        address: Option<usize>,
        mode: ProcessorMode,
        flags: u32,
    ) -> Result<PVOID, NtStatus> {
        let address = match microseh::try_seh(|| unsafe {
            MmMapLockedPagesSpecifyCache(
                self.mdl.ptr,
                mode,
                MemoryCacheType::MmCached,
                address.unwrap_or(0) as _,
                Boolean::False,
                flags,
            )
        }) {
            Ok(ptr) => match ptr as usize {
                0 => {
                    let mut logger = GLOBAL_LOGGER.lock();
                    logger.error(LogEvent::FailedToAllocate);
                    return Err(NtStatus::NotAllocated);
                }
                x => x,
            },
            Err(err) => {
                let mut logger = GLOBAL_LOGGER.lock();
                logger.error(LogEvent::Exception(err.code() as _));
                return Err(NtStatus::AccessViolation);
            }
        };

        self.status = MapStatus::Mapped(address as _);
        Ok(address as _)
    }
}
