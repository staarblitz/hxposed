use core::{ptr::null_mut, sync::atomic::{AtomicBool, AtomicPtr, Ordering}, task::Waker};

use alloc::boxed::Box;
use spin::Mutex;
use crate::hxposed::ProcessObject;
use atomic_enum::atomic_enum;

#[derive(Default, Debug)]
pub struct UnsafeAsyncInfo {
    pub handle: u64,
    pub result_values: *mut [u64; 4],
    pub process: ProcessObject
}

unsafe impl Send for UnsafeAsyncInfo {}
unsafe impl Sync for UnsafeAsyncInfo {}

impl UnsafeAsyncInfo {
    pub fn is_present(&self) -> bool {
        self.handle != 0
    }
}


#[cfg(feature = "usermode")]
pub mod async_service;

#[derive(Debug, Default)]
pub struct WakerCell {
    ptr: AtomicPtr<Waker>,
}


impl WakerCell {
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
        }
    }

    pub fn register(&self, w: &Waker) {
        let boxed = Box::into_raw(Box::new(w.clone()));
        let prev = self.ptr.swap(boxed, Ordering::AcqRel);
        if !prev.is_null() {
            // SAFETY: prev was a Box<Waker> allocated above
            unsafe { drop(Box::from_raw(prev)) };
        }
    }

    pub fn take(&self) -> Option<Waker> {
        let p = self.ptr.swap(null_mut(), Ordering::AcqRel);
        if p.is_null() {
            None
        } else {
            // SAFETY: p is Box<Waker>
            let boxed = unsafe { Box::from_raw(p) };
            Some(*boxed)
        }
    }

    pub fn wake_and_clear(&self) {
        if let Some(w) = self.take() {
            w.wake();
        }
    }
}

impl Drop for WakerCell {
    fn drop(&mut self) {
        let p = self.ptr.load(Ordering::Acquire);
        if !p.is_null() {
            unsafe { drop(Box::from_raw(p)) };
        }
    }
}

#[derive(Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    pub result_values: Mutex<Box<[u64; 4]>>,
    pub state: AtomicPromiseState,
    pub waker: WakerCell,
}

#[atomic_enum]
pub enum PromiseState {
    None,
    Waiting,
    Completed,
    TimedOut,
    CancelPending
}