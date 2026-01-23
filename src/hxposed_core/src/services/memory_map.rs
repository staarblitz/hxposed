use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::memory::*;
use crate::services::process::HxProcess;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
///
/// # HxMemoryDescriptor<T>
///
/// Abstraction over MDL structure.
///
/// You can access the inner fields, but it's recommended for you to not do that.
pub struct HxMemoryDescriptor<T> {
    pub memory_type: MemoryType,
    pub system_pa: u64,
    pub length: u32,
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
pub struct HxMemoryGuard<'process, T> {
    pub virtual_addr: *mut T,
    pub kernel_mem: &'process HxMemoryDescriptor<T>,
    pub process: &'process HxProcess,
}

impl<'a, T> Deref for HxMemoryGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.virtual_addr }
    }
}

impl<'a, T> DerefMut for HxMemoryGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.virtual_addr }
    }
}

impl<'a, T> Drop for HxMemoryGuard<'a, T> {
    fn drop(&mut self) {
        let _ = self.process.memory.set_attributes(
            PagingType::from_va(Va::from(self.virtual_addr as u64)),
            PageAttributes::Present(false),
        );
    }
}

impl<T> Drop for HxMemoryDescriptor<T> {
    fn drop(&mut self) {
        let _ = FreeMemoryRequest {
            system_va: self.system_pa,
            memory_type: self.memory_type,
        }
        .send();
    }
}

impl<T> HxMemoryDescriptor<T> {
    pub fn map<'a>(
        &'a self,
        process: &'a HxProcess,
        address: u64,
    ) -> Result<HxMemoryGuard<T>, HypervisorError> {
        MapVaToPaRequest {
            addr_space: process.addr,
            phys: self.system_pa,
            virt: address,
        }
        .send()?;

        Ok(HxMemoryGuard::<T> {
            virtual_addr: address as _,
            kernel_mem: self,
            process,
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
    pub(crate) fn new(memory_type: MemoryType, system_pa: u64, length: u32) -> Self {
        Self {
            memory_type,
            system_pa,
            length,
            phantom: PhantomData,
        }
    }
}
