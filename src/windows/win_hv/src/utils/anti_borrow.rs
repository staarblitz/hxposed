use alloc::boxed::Box;

/// For delicate moments when it simply just doesn't work, but you can prove it will.
///
/// # AntiBorrow<T>
///
/// Stores a [`T`]. Allows you to have multiple mutable references to it.
///
/// You can use this in annoying situations like trying to get mutable references to different objects in a [`Vec`]
pub struct AntiBorrow<T> {
    data: *const T,
}

impl<T: Default> Default for AntiBorrow<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> AntiBorrow<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Box::into_raw(Box::new(data)),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.data as *mut T
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe { &mut *(self.as_mut_ptr()) }
    }

    #[allow(invalid_reference_casting)]
    pub fn replace(&self, new: T) {
        unsafe {
            drop(Box::from_raw(self.as_mut_ptr()));
        }

        unsafe { &mut *(self as *const AntiBorrow<T> as *mut AntiBorrow<T>) }.data =
            Box::into_raw(Box::new(new));
    }
}
