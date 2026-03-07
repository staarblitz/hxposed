use core::ops::{Deref, DerefMut};

/// Purely for convenience. Unsafe. Must not be used if caller doesn't guarantee ptr is valid for lifetime of DangerPtr
#[derive(Debug)]
pub struct DangerPtr<T> {
    pub ptr: *mut T,
}

impl<T> PartialEq<Self> for DangerPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Deref for DangerPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for DangerPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
