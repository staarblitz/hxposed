use crate::error::HypervisorError;
use crate::hxposed::requests::memory::{FreeMemoryRequest, MapMemoryOperation, MapMemoryRequest};
use crate::hxposed::requests::Vmcall;
use crate::hxposed::responses::HypervisorResponse;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::memory::HxMemory;
use crate::services::process::HxProcess;
use crate::services::types::memory_fields::{KernelMemoryState, MemoryPool};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct HxMemoryDescriptor<T> {
    pub pool: MemoryPool,
    pub mapped_addr: u64,
    pub length: u32,
    pub state: KernelMemoryState,
    phantom: PhantomData<T>,
}

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
        // safety: safe due to lifetimes.
        let _ = self.unmap();
    }
}

impl<T> Drop for HxMemoryDescriptor< T> {
    fn drop(&mut self) {
        // safety: safe due to lifetimes.
        let _ = self.free();
    }
}

impl<'guard, T> HxMemoryGuard<'guard, T> {
    async fn unmap(&mut self) {
        let _ = MapMemoryRequest {
            original_system_va: self.kernel_mem.mapped_addr,
            map_address: self.virtual_addr as u64,
            operation: MapMemoryOperation::Unmap,
            process: self.process,
        }
        .send_async()
        .await;

        self.kernel_mem.state = KernelMemoryState::Allocated;
    }
}

impl<T> HxMemoryDescriptor<T> {
    ///
    /// # DO NOT USE. USE [`HxMemory::alloc`] INSTEAD.
    ///
    /// # New
    ///
    /// This is an abstraction over the kernel `_MDL` structure.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    /// ## Arguments
    /// * `mapped_addr` - Virtual address to describe.
    /// * `length` - Length of the range.
    /// * `pool` - Kind of pool to allocate from. See [`MemoryPool`].
    /// * `state` - Current state of the descriptor. To describe a physical range manually, set this to [`KernelMemoryState::None`]. See [`KernelMemoryState`].
    ///
    /// ## Remarks
    /// - Memory is NOT allocated upon return.
    /// - You should use [`HxMemory::alloc`] to allocate physical memory.
    pub(crate) fn new(
        mapped_addr: u64,
        length: u32,
        pool: MemoryPool,
        state: KernelMemoryState,
    ) -> Self {
        Self {
            mapped_addr,
            length,
            pool,
            state,
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
    pub async fn map<'guard>(
        &'_ mut self,
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
            original_system_va: self.mapped_addr,
            map_address: map_address.unwrap_or(0),
            operation: MapMemoryOperation::Map,
            process: process_addr,
        }
        .send_async()
        .await?;

        Ok(HxMemoryGuard::<'_, T> {
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
    async fn free(&self) -> Result<(), ()> {
        if self.state == KernelMemoryState::Mapped {
            return Err(());
        }

        let _ = FreeMemoryRequest {
            original_system_va: self.mapped_addr,
        }
        .send_async()
        .await;

        Ok(())
    }
}
