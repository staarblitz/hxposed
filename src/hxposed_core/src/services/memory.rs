use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::{ProcessMemoryOperation, ProtectProcessMemoryRequest, RWProcessMemoryRequest};
use crate::plugins::plugin_perms::PluginPermissions;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::copy_nonoverlapping;
use crate::hxposed::responses::process::RWProcessMemoryResponse;
use crate::intern::win::GetCurrentProcessId;
use crate::services::types::memory_fields::MemoryProtection;

pub struct HxMemory {
    pub id: u32,
}

impl HxMemory {
    pub fn current() -> HxMemory{
        HxMemory { id: unsafe {GetCurrentProcessId()} }
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
        }.send_async().await?;

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
    pub async fn read<T>(
        &self,
        address: *mut u8,
        count: usize,
    ) -> Result<Vec<T>, HypervisorError> {
        let mut raw = vec![0u8; count * size_of::<T>()];

        let result = RWProcessMemoryRequest {
            id: self.id,
            address: address as _,
            count: count * size_of::<T>(),
            data: raw.as_mut_ptr(),
            data_len: count * size_of::<T>(),
            operation: ProcessMemoryOperation::Read
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
            operation: ProcessMemoryOperation::Write
        }.send_async().await?;

        Ok(result.bytes_processed)
    }
}
