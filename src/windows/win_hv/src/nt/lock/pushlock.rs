use wdk_sys::ntddk::{
    ExAcquirePushLockExclusiveEx, ExAcquirePushLockSharedEx, ExReleasePushLockExclusiveEx,
    ExReleasePushLockSharedEx,
};

#[derive(Debug)]
pub struct PushLock {
    lock: *mut u64,
}

unsafe impl Sync for PushLock {}
unsafe impl Send for PushLock {}

impl PushLock {
    pub unsafe fn from_ptr(ptr: *mut u64) -> Self {
        Self { lock: ptr }
    }

    pub fn acquire_exclusive(&mut self) -> PushLockGuard {
        PushLockGuard::new(self.lock, LockType::Exclusive)
    }

    pub fn acquire_shared(&self) -> PushLockGuard {
        PushLockGuard::new(self.lock, LockType::Shared)
    }
}

pub enum LockType {
    Exclusive,
    Shared,
}

pub struct PushLockGuard {
    lock: *mut u64,
    lock_type: LockType,
}

impl Drop for PushLockGuard {
    fn drop(&mut self) {
        match self.lock_type {
            LockType::Exclusive => unsafe { ExReleasePushLockExclusiveEx(self.lock, 0) },
            LockType::Shared => unsafe { ExReleasePushLockSharedEx(self.lock, 0) },
        }
    }
}

impl PushLockGuard {
    pub(crate) fn new(lock: *mut u64, lock_type: LockType) -> Self {
        let me = Self { lock, lock_type };

        match me.lock_type {
            LockType::Exclusive => unsafe { ExAcquirePushLockExclusiveEx(lock, 0) },
            LockType::Shared => unsafe { ExAcquirePushLockSharedEx(lock, 0) },
        }

        me
    }
}
