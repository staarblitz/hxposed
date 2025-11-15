use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicPtr, Ordering};

pub(crate) mod w25h2;

pub trait UncheckedAsMut<T> {
    #[inline]
    fn unchecked_as_mut(&self) -> &'static mut T;

    #[inline]
    /// Same as [unchecked_as_mut], shortened.
    fn uam(&self) -> &'static mut T;
}

impl<T> UncheckedAsMut<T> for *mut T {
    #[inline]
    // not as_mut_unchecked because that already exists.
    fn unchecked_as_mut(&self) -> &'static mut T {
        unsafe { self.as_mut().unwrap() }
    }

    fn uam(&self) -> &'static mut T {
        self.unchecked_as_mut()
    }
}