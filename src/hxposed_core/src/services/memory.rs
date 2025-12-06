use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::intern::win::GetCurrentProcessId;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::types::memory_fields::{KernelMemoryState, MemoryPool, MemoryProtection};
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ptr::copy_nonoverlapping;
use crate::hxposed::call::ServiceParameter;
use crate::hxposed::requests::memory::*;
use crate::hxposed::responses::HypervisorResponse;
use crate::services::memory_map::HxMemoryDescriptor;

pub struct HxMemory {
    pub id: u32,
}

impl HxMemory {
    ///
    /// # Current Memory
    ///
    /// Opens instance of HxMemory for current process.
    pub fn current() -> HxMemory {
        HxMemory {
            id: unsafe { GetCurrentProcessId() },
        }
    }

    ///
    /// # Allocate<T>
    ///
    /// Allocates memory from kernel non-paged pool enough to hold an instance of [`T`].
    ///
    /// ## Arguments
    /// * `pool` - Kind of pool to allocate from. See [`MemoryPool`].
    ///
    /// ## Remarks
    /// - This function allocates from kernel memory. Use with caution!
    /// - The memory is only ALLOCATED, not MAPPED. Use [`HxMemoryDescriptor::map`] to map memory into a process' address space.
    /// - The allocation will NOT be freed when process exits. You can access your existing allocations using [`Self::get_allocs`].
    /// - The align value is discarded, currently.
    /// - Size of [`T`] must not be bigger than [`u32::MAX`].
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_PHYSICAL`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    ///
    /// ## Return
    /// * [`HxMemoryDescriptor<T>`] - An abstract representation of the allocation. See [`HxMemoryDescriptor`].
    /// * [`HypervisorError`] - Most likely an NT error telling there is not enough memory caused by your blunders using this framework.
    ///
    /// ## Example
    /// ```rust
    /// let alloc = HxMemory::alloc<u8>(MemoryPool::NonPaged).unwrap();
    /// {
    ///     let map = alloc.map().unwrap();
    ///     *map.as_mut() = 5;
    /// } // automatically unmaps
    ///
    /// ```
    pub async fn alloc<T>(pool: MemoryPool) -> Result<HxMemoryDescriptor<T>, HypervisorError> {
        let size = size_of::<T>();
        if size > u32::MAX as usize {
            return Err(HypervisorError::from_response(HypervisorResponse::invalid_params(ServiceParameter::Arg2)))
        }

        let align = align_of::<T>();

        let alloc = AllocateMemoryRequest {
            size: size as u32,
            align,
            pool,
        }
        .send_async()
        .await?;

        Ok(HxMemoryDescriptor::<T>::new(alloc.address, alloc.bytes_allocated, pool, KernelMemoryState::Allocated))
    }

    ///
    /// # Protect Memory
    ///
    /// Changes page protection.
    ///
    /// ## Arguments
    /// * `address` - Virtual address of the page.
    /// * `protection` - Protection to apply. See [`MemoryProtection`]
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_MEMORY`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    /// - [`PluginPermissions::MEMORY_PROTECT`]
    ///
    /// ## Returns
    /// * [`MemoryProtection`] - Returns the old protection.
    /// * [`HypervisorError`] - Any error `ProtectVirtualMemory` can return.
    ///
    pub async fn protect(
        &mut self,
        address: *mut u8,
        protection: MemoryProtection,
    ) -> Result<MemoryProtection, HypervisorError> {
        let result = ProtectProcessMemoryRequest {
            id: self.id,
            address: address as _,
            protection,
        }
        .send_async()
        .await?;

        Ok(result.old_protection)
    }

    ///
    /// # Read Memory
    ///
    /// Reads specified amount of memory from specified address.
    ///
    /// ## Arguments
    /// * `address` - Address of memory to begin reading from.
    /// * `count` - Number of **items** to read.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_MEMORY`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    /// ## Returns
    /// * [`Vec<T>`] - Number of items read.
    /// * [`HypervisorError`] - Any error `ReadProcessMemory` can return.
    ///
    pub async fn read<T>(&self, address: *mut u8, count: usize) -> Result<Vec<T>, HypervisorError> {
        let mut raw = vec![0u8; count * size_of::<T>()];

        let result = RWProcessMemoryRequest {
            id: self.id,
            address: address as _,
            count: count * size_of::<T>(),
            data: raw.as_mut_ptr(),
            data_len: count * size_of::<T>(),
            operation: ProcessMemoryOperation::Read,
        }
        .send_async()
        .await?;

        let ptr = raw.as_ptr() as *mut T;
        let len = result.bytes_processed / size_of::<T>();

        let mut out = Vec::with_capacity(len);
        unsafe {
            out.set_len(len);
            copy_nonoverlapping(ptr, out.as_mut_ptr(), len);
        }

        Ok(out)
    }

    ///
    /// # Write Memory
    ///
    /// Writes specified amount of memory to specified address.
    ///
    /// ## Arguments
    /// * `address` - Address of memory to write to.
    /// * `data` - Source memory.
    /// * `count` - Number of **items** to read.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_MEMORY`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    ///
    /// ## Returns
    /// * [`usize`] - Contains number of bytes read.
    /// * [`HypervisorError`] - Any error `ReadProcessMemory` can return.
    ///
    pub async fn write<T>(
        &mut self,
        address: *mut u8,
        data: *const T,
        count: usize,
    ) -> Result<usize, HypervisorError> {
        let result = RWProcessMemoryRequest {
            id: self.id,
            address: address as _,
            count: count * size_of::<T>(),
            data: data as _,
            data_len: count * size_of::<T>(),
            operation: ProcessMemoryOperation::Write,
        }
        .send_async()
        .await?;

        Ok(result.bytes_processed)
    }
}
