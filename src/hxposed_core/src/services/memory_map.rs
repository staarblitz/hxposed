use crate::error::HypervisorError;
use crate::hxposed::RmdObject;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::memory::*;
use crate::hxposed::responses::memory::PageAttributeResponse;
use crate::services::process::HxProcess;
use core::arch::asm;
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
    pub rmd: RmdObject,
    pub length: u32,
    phantom: PhantomData<T>,
    pub owns: bool,
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
    pub va: Va,
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
        self.unmap();
    }
}

impl<'a, T> HxMemoryGuard<'a, T> {
    fn unmap(&mut self) {
        MapRmdRequest {
            object: self.kernel_mem.rmd,
            addr_space: self.process.addr,
            map_addr: self.va.into(),
            operation: MapOperation::Unmap,
        }
        .send()
        .unwrap();
    }
}

impl<T> Drop for HxMemoryDescriptor<T> {
    fn drop(&mut self) {
        if self.owns {
            let _ = FreeMemoryRequest { obj: self.rmd }.send();
        }
    }
}

impl<T> HxMemoryDescriptor<T> {
    pub fn map<'a>(
        &'a self,
        process: &'a HxProcess,
        address: u64,
    ) -> Result<HxMemoryGuard<T>, HypervisorError> {
        MapRmdRequest {
            addr_space: process.addr,
            object: self.rmd,
            map_addr: address,
            operation: MapOperation::Map,
        }
        .send()?;

        Ok(HxMemoryGuard::<T> {
            virtual_addr: address as _,
            kernel_mem: self,
            va: Va::from(address),
            process,
        })
    }

    ///
    /// # New
    ///
    /// A new instance of [`HxMemoryDescriptor`]. Abstraction over `_MDL`
    ///
    /// ## Arguments
    /// * `rmd` - Raw Memory Descriptor object.
    /// * `length` - Length of the described data
    ///
    /// ## Remarks
    /// - Memory is NOT allocated upon return.
    /// - You should use [`HxMemory::alloc`] to allocate physical memory.
    /// - This merely returns an instance of [`HxMemoryDescriptor`]. No kernel involved.
    pub fn new(rmd: u64, length: u32) -> Self {
        Self {
            rmd,
            length,
            phantom: PhantomData,
            owns: true
        }
    }
}
