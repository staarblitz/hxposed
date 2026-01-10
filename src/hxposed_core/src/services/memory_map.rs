use crate::error::HypervisorError;
use crate::hxposed::requests::memory::{AllocateMemoryRequest, FreeMemoryRequest, MapMemoryOperation, MapMemoryRequest};
use crate::hxposed::requests::Vmcall;
use crate::hxposed::responses::HypervisorResponse;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::memory::HxMemory;
use crate::services::process::HxProcess;
use crate::services::types::memory_fields::{KernelMemoryState, MemoryPool};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use crate::hxposed::MdlObject;
use crate::hxposed::responses::memory::MapMemoryResponse;

#[derive(Debug)]
///
/// # HxMemoryDescriptor<T>
///
/// Abstraction over MDL structure.
///
/// You can access the inner fields, but it's recommended for you to not do that.
pub struct HxMemoryDescriptor<T> {
    pub pool: MemoryPool,
    pub mdl_addr: MdlObject,
    pub length: u32,
    pub state: KernelMemoryState,
    pub mapped_addr: usize,
    phantom: PhantomData<T>,
}

unsafe impl<T> Sync for HxMemoryDescriptor<T> {}
unsafe impl<T> Send for HxMemoryDescriptor<T> {}

#[derive(Debug)]
///
/// # HxMemoryGuard<T>
///
/// A scope for allowing access to a mapped piece of memory.
///
/// ## Remarks
/// If this isn't mapped to current process, using this structure will result in segmentation fault.
/// You have 2 options:
/// 1. Create a new [`HxMemoryDescriptor`] that describes same pages using [`HxMemoryDescriptor::new_describe`]. And map it into your own address space.
/// 2. Use [`HxMemory::write`] and [`HxMemory::read`].
///
/// Personally I would choose the first one.
pub struct HxMemoryGuard<'guard,T> {
    pub virtual_addr: *mut T,
    pub kernel_mem: &'guard mut HxMemoryDescriptor<T>,
    pub process: u64,
}

impl<'guard, T> Deref for HxMemoryGuard<'guard, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.virtual_addr }
    }
}

impl<'guard, T> DerefMut for HxMemoryGuard<'guard, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.virtual_addr }
    }
}

impl<'guard, T> Drop for HxMemoryGuard<'guard, T> {
    fn drop(&mut self) {
    }
}

impl<T> Drop for HxMemoryDescriptor< T> {
    fn drop(&mut self) {
        if self.state != KernelMemoryState::None {
        }
    }
}

impl<'guard, T> HxMemoryGuard<'guard, T> {
    pub async fn unmap(&mut self) -> Result<MapMemoryResponse, HypervisorError> {
        let x = MapMemoryRequest {
            mdl: self.kernel_mem.mdl_addr,
            map_address: self.virtual_addr as u64,
            operation: MapMemoryOperation::Unmap,
            process: self.process,
        }
        .send_async()
        .await?;

        self.kernel_mem.state = KernelMemoryState::Allocated;
        Ok(x)
    }
}

impl<T> HxMemoryDescriptor<T> {

    ///
    /// # New Describe
    ///
    /// Allocates an MDL describing `virtual_addr`.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    /// ## Arguments
    /// * `virtual_addr` - Underlying pages to describe.
    /// * `size` - Size of the pages to describe.
    ///
    /// ## Safety
    /// - Caller must ensure `virtual_addr` and its length is valid until lifetime of the descriptor.
    ///
    /// ## Return
    /// * [`Self`] - State is allocated. Not mapped.
    /// * [`HypervisorError`] - Error
    pub async fn new_describe(virtual_addr: *const u8, size: u32) -> Result<Self, HypervisorError> {
        let mdl = AllocateMemoryRequest {
            size,
            underlying_pages: virtual_addr as _,
            pool: MemoryPool::NonPaged
        }.send_async().await?;

        Ok(Self {
            mdl_addr: mdl.mdl,
            pool: MemoryPool::NonPaged,
            length: size,
            state: KernelMemoryState::Allocated,
            phantom: PhantomData,
            mapped_addr: virtual_addr as _,
        })
    }

    ///
    /// # Clone
    ///
    /// Allocates a new MDL that describes the same underlying pages.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    /// ## Safety
    /// - Caller must ensure `virtual_addr` and its length is valid until lifetime of the descriptor.
    ///
    pub async fn clone(&self) -> Result<Self, HypervisorError> {
        let mdl = AllocateMemoryRequest {
            size: self.length,
            underlying_pages: self.mapped_addr,
            pool: self.pool.clone(),
        }.send_async().await?;

        Ok(Self {
            mdl_addr: mdl.mdl,
            pool: self.pool.clone(),
            length: self.length,
            state: KernelMemoryState::Allocated,
            mapped_addr: self.mapped_addr,
            phantom: PhantomData,
        })
    }

    ///
    /// # New
    ///
    /// This is an abstraction over the kernel `_MDL` structure.
    ///
    /// ## Remarks
    /// - Memory is NOT allocated upon return.
    /// - You should use [`HxMemory::alloc`] to allocate physical memory.
    /// - This merely returns an instance of [`HxMemoryDescriptor`]. No kernel involved.
    ///
    /// ## Arguments
    /// * `mdl_addr` - address of MDL object.
    /// * `length` - Length of the range.
    /// * `pool` - Kind of pool to allocate from. See [`MemoryPool`].
    /// * `state` - Current state of the descriptor. <strike>To describe a physical range manually, set this to [`KernelMemoryState::None`]. See [`KernelMemoryState`]</strike>.
    ///
    pub(crate) fn new(
        mdl_addr: u64,
        length: u32,
        pool: MemoryPool,
        state: KernelMemoryState,
    ) -> Self {
        Self {
            mdl_addr,
            length,
            pool,
            state,
            mapped_addr: 0,
            phantom: PhantomData,
        }
    }

    ///
    /// # Map
    ///
    /// Maps the memory into address space of specified process. (only to current process for now)
    ///
    /// ## Arguments
    /// * `process` - Pointer to [`HxProcess`] to map the memory into. Set as [`None`] to map to current process.
    /// * `map_address` - Virtual address to map the MDL into. Set as [`None`] to let system determine.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    /// - [`PluginPermissions::MEMORY_PHYSICAL`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    ///
    /// - If `process` argument is [`Some`], then following permissions are required:
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    /// - [`PluginPermissions::PROCESS_MEMORY`]
    ///
    /// ## Remarks
    /// - When [`HxMemoryGuard`] goes out of scope, it automatically unmaps the memory.
    /// - Mapping to address space of another process means you **DO NOT** possess access to it.
    ///
    /// ## Returns
    /// * [`HxMemoryGuard<T>`] - Describes the mapped memory. You should NOT use or dereference it if it's mapped to address space of another process.
    pub async fn map(
        &mut self,
        process: Option<&mut HxProcess>,
        map_address: Option<u64>,
    ) -> Result<HxMemoryGuard<'_, T>, HypervisorError> {
        if self.state != KernelMemoryState::Allocated {
            return Err(
                HypervisorError::from_response(HypervisorResponse::nt_error(0xC00000A0))
            );
        }

        let process_addr = match process {
            Some(ref p) => p.addr,
            None => 0,
        };

        let result = MapMemoryRequest {
            mdl: self.mdl_addr,
            map_address: map_address.unwrap_or(0),
            operation: MapMemoryOperation::Map,
            process: process_addr,
        }
        .send_async()
        .await?;

        Ok(HxMemoryGuard {
            virtual_addr: result.mapped_address as *mut T,
            kernel_mem: self,
            process: process_addr,
        })
    }

    /// Free while it's still mapped, I dare you.
    /// Do the world a big favor and teach us what happens when we try to free a memory that is mapped.
    /// Will it crash? Will it bugcheck? Will it work? Will it throw access violations?
    ///
    /// I don't know. I don't want to learn.
    pub async fn free(&mut self) -> Result<(), ()> {
        match self.state {
            KernelMemoryState::Mapped(_) => {
                return Err(());
            }
            _ => {}
        }

        let _ = FreeMemoryRequest {
            mdl: self.mdl_addr,
        }
            .send_async()
            .await;

        self.state = KernelMemoryState::None;

        Ok(())
    }
}
