use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::memory::{FreeMemoryRequest, MapMemoryOperation, MapMemoryRequest};
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::memory::HxMemory;
use crate::services::types::memory_fields::{KernelMemoryState, MemoryPool};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use spin::Mutex;

pub struct HxMemoryDescriptor<T> {
    pub pool: MemoryPool,
    pub mdl_address: u64,
    pub mapped_addr: u64,
    pub length: u32,
    pub state: KernelMemoryState,
    phantom: PhantomData<T>,
}

pub struct HxMemoryGuard<'guard, T> {
    pub virtual_addr: *mut T,
    pub kernel_mem: &'guard mut HxMemoryDescriptor<T>,
    pub mutex: Mutex<()>,
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
        self.kernel_mem.unmap();
    }
}

impl<'descriptor, T> HxMemoryDescriptor<T> {
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
    /// - I have no idea how to implement that. Do yourself a favor and don't use this.
    pub fn new(mapped_addr: u64, length: u32, pool: MemoryPool, state: KernelMemoryState) -> Self {
        Self {
            mapped_addr,
            mdl_address: mapped_addr,
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
    /// * `map_address` - Virtual address to map the MDL into.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    /// - [`PluginPermissions::MEMORY_PHYSICAL`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    /// ## Remarks
    /// - When [`HxMemoryGuard`] goes out of scope, it automatically unmaps the memory.
    ///
    pub async fn map(
        &'descriptor mut self,
        map_address: Option<u64>,
    ) -> Result<HxMemoryGuard<'descriptor, T>, HypervisorError> {
        let result = MapMemoryRequest {
            mdl_address: self.mdl_address,
            map_address: map_address.unwrap_or(0),
            operation: MapMemoryOperation::Map,
        }
        .send_async()
        .await?;

        Ok(HxMemoryGuard::<'descriptor, T> {
            virtual_addr: result.mapped_address as *mut T,
            kernel_mem: self,
            mutex: Mutex::new(()),
        })
    }

    /// Free while it's still mapped, I dare you.
    /// Do the world a big favor and teach us what happens when we try to free a memory that is mapped.
    /// Will it crash? Will it bugcheck? Will it work? Will it throw access violations?
    ///
    /// I don't know. I don't want to learn.
    pub async fn free(self) {
        let _ = FreeMemoryRequest {
            mdl_address: self.mdl_address,
        }
        .send_async()
        .await;
    }

    pub async fn unmap(&mut self) {
        let _ = MapMemoryRequest {
            mdl_address: self.mdl_address,
            map_address: self.mapped_addr,
            operation: MapMemoryOperation::Map,
        }
        .send_async()
        .await;
    }
}
