use microseh::Exception;

pub trait SehRead<T> {
    unsafe fn seh_read(self) -> Result<T, Exception>;
}

pub trait SehWrite<T> {
    unsafe fn seh_write(self, value: T) -> Result<(), Exception>;
}

impl<T> SehRead<T> for *const T {
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn seh_read(self) -> Result<T, Exception> {
        microseh::try_seh(|| self.read())
    }
}

impl<T> SehWrite<T> for *mut T {
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn seh_write(self, value: T) -> Result<(), Exception> {
        let mut value = Some(value);
        microseh::try_seh(|| {
            let v = value.take().unwrap();
            self.write(v)
        })
    }
}
