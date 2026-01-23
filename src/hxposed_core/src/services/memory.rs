#![allow(dead_code)]

use crate::error::HypervisorError;
use crate::hxposed::call::ServiceParameter;
use crate::hxposed::ProcessObject;
use crate::hxposed::requests::memory::*;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::HypervisorResponse;
use crate::hxposed::responses::memory::PageAttributeResponse;
use crate::services::memory_map::HxMemoryDescriptor;
use crate::services::process::HxProcess;

#[derive(Debug)]
pub struct HxMemory<> {
    pub process: ProcessObject
}

impl HxMemory {

    pub fn get_attributes(&self, page_type: PagingType, attribute: PageAttributes) -> Result<PageAttributeResponse, HypervisorError> {
        PageAttributeRequest {
            addr_space: self.process,
            operation: PageAttributeOperation::Get,
            paging_type: page_type,
            attributes: attribute
        }.send()
    }

    pub fn set_attributes(&self, page_type: PagingType, attribute: PageAttributes) -> Result<PageAttributeResponse, HypervisorError> {
        PageAttributeRequest {
            addr_space: self.process,
            operation: PageAttributeOperation::Set,
            paging_type: page_type,
            attributes: attribute
        }.send()
    }

    pub fn map(&self, va: u64, pa: u64) -> Result<EmptyResponse, HypervisorError> {
        MapVaToPaRequest {
            addr_space: self.process,
            virt: va,
            phys: pa,
        }.send()
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
    pub fn alloc<T>(memory_type: MemoryType) -> Result<HxMemoryDescriptor<T>, HypervisorError> {
        if size_of::<T>() > u32::MAX as usize {
            return Err(HypervisorError::from_response(
                HypervisorResponse::invalid_params(ServiceParameter::Arg2),
            ));
        }

        let result = Self::alloc_raw(size_of::<T>() as _, memory_type)?;

        Ok(HxMemoryDescriptor::<T>::new(
           memory_type,
           result,
           size_of::<T>() as _
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
    /// * [`HypervisorError`] - Most likely an NT error telling there is not enough memory caused by your blunders using this framework.
    pub fn alloc_raw(size: u32, memory_type: MemoryType) -> Result<u64, HypervisorError> {
        Ok(AllocateMemoryRequest { size, memory_type }
            .send()?
            .system_pa)
    }
}
