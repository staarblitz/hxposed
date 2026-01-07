/// Just because borrow checker is an ass.
/// SAFETY: unsafe. Do not use in production. Solely for test.
pub struct ExtremeCell<T> {
    data: *const T,
}

impl<T> ExtremeCell<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Box::into_raw(Box::new(data)),
        }
    }

    pub fn get_mut(&self) -> *mut T {
        self.data as *mut T
    }

    pub fn get_const(&self) -> *const T {
        self.data
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe { &mut *(self.get_mut()) }
    }

    pub fn as_const(&self) -> &T {
        unsafe { &*(self.get_const()) }
    }

    pub fn drop_inner(&self) {
        drop(unsafe { Box::from_raw(self.get_mut()) });
    }

    #[allow(invalid_reference_casting)]
    pub fn replace(&self, new: T) {
        unsafe {
            drop(Box::from_raw(self.get_mut()));
        }

        unsafe { &mut *(self as *const ExtremeCell<T> as *mut ExtremeCell<T>) }.data =
            Box::into_raw(Box::new(new));
    }
}
