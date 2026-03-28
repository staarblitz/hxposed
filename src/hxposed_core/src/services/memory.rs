#![allow(dead_code)]

use crate::error::HxError;
use crate::hxposed::requests::memory::*;
use crate::hxposed::requests::Syscall;
use crate::hxposed::responses::memory::PageAttributeResponse;
use crate::hxposed::responses::HxResponse;
use crate::hxposed::ProcessObject;
use crate::services::memory_map::HxMemoryDescriptor;

#[derive(Debug)]
pub struct HxMemory {
    pub process: ProcessObject,
}

impl HxMemory {
    pub fn get_paging_type(
        &self,
        page_type: PagingType,
    ) -> Result<PageAttributeResponse, HxError> {
        PageAttributeRequest {
            addr_space: self.process,
            operation: PageAttributeOperation::Get,
            paging_type: page_type,
            type_bits: 0
        }
        .send()
    }

    pub fn set_paging_type(
        &self,
        page_type: PagingType,
        type_bits: u64,
    ) -> Result<PageAttributeResponse, HxError> {
        PageAttributeRequest {
            addr_space: self.process,
            operation: PageAttributeOperation::Set,
            paging_type: page_type,
            type_bits,
        }
        .send()
    }

    pub fn translate_addr(
        // huh?
        process: crate::services::process::HxProcess,
        addr: u64
    ) -> Result<u64, HxError> {
        let k = TranslateAddressRequest {
            virtual_addr: addr,
            addr_space: process.addr,
        }.send()?;

        Ok(k.physical_addr)
    }

    ///
    /// # Allocate<T>
    ///
    /// Allocates memory from kernel big enough to hold an instance of [`T`] in **system address space** process.
    /// - To allocate memory on address space of another process, see [`Self::alloc_on_behalf`].
    ///
    /// ## Arguments
    /// * `pool` - Kind of pool to allocate from. See [`MemoryPool`].
    ///
    /// ## Remarks
    /// - This function allocates from kernel memory. Use with caution!
    /// - The memory is only ALLOCATED, not MAPPED. Use [`HxMemoryDescriptor::map`] to map memory into a process' address space.
    /// - Size of [`T`] must not be bigger than [`u32::MAX`].
    ///
    /// ## Permissions
    /// - [`PluginPermissions::MEMORY_PHYSICAL`]
    /// - [`PluginPermissions::MEMORY_VIRTUAL`]
    /// - [`PluginPermissions::MEMORY_ALLOCATION`]
    ///
    /// ## Return
    /// * [`HxMemoryDescriptor<T>`] - An abstract representation of the allocation. See [`HxMemoryDescriptor`].
    /// * [`HxError`] - Most likely an NT error telling there is not enough memory caused by your blunders using this framework.
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
    pub fn alloc<T>(memory_type: MemoryType) -> Result<HxMemoryDescriptor<T>, HxError> {
        if size_of::<T>() > u32::MAX as usize {
            return Err(HxError::InvalidParameters(0));
        }

        let result = Self::alloc_raw(size_of::<T>() as _, memory_type)?;

        Ok(HxMemoryDescriptor::<T>::new(
            result,
            size_of::<T>() as _,
        ))
    }

    ///
    /// # Alloc Raw
    ///
    /// Makes a raw kernel allocation.
    ///
    /// Do not use this function unless you have to. See [`Self::alloc`].
    ///
    /// ## Arguments
    /// * `size` - Number of bytes to allocate.
    /// * `memory_type` - Kind of pool to allocate from. See [`MemoryType`].
    ///
    /// ## Remarks
    /// - All remarks that apply to other alloc* functions.
    /// - This does NOT return a HxMemoryGuard. You are on your own.
    ///
    /// ## Return
    /// * [`u64`] - Allocated region's system address.
    /// * [`HxError`] - Most likely an NT error telling there is not enough memory caused by your blunders using this framework.
    pub fn alloc_raw(size: u32, memory_type: MemoryType) -> Result<u64, HxError> {
        Ok(AllocateMemoryRequest { size, memory_type }
            .send()?
            .rmd)
    }
}
