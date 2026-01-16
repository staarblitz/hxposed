use crate::nt::process::NtProcess;
use crate::objects::ObjectTracker;
use crate::services::commands::AsyncCommand;
use crate::services::commands::callback::AwaitNotificationRequestAsyncCommand;
use crate::utils::rng::SimpleCounter;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use core::hash::{Hash, Hasher};
use hxposed_core::hxposed::requests::notify::ObjectState;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::notify::AwaitNotificationResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::{CallbackObject, ObjectType};
use spin::{Mutex, RwLock};
use wdk_sys::_PSCREATETHREADNOTIFYTYPE::PsCreateThreadNotifyNonSystem;
use wdk_sys::ntddk::{
    ObRegisterCallbacks, PsLookupThreadByThreadId, PsSetCreateProcessNotifyRoutineEx,
    PsSetCreateThreadNotifyRoutineEx,
};
use wdk_sys::{
    BOOLEAN, FALSE, HANDLE, NTSTATUS, OB_CALLBACK_REGISTRATION, PEPROCESS, PPS_CREATE_NOTIFY_INFO,
    STATUS_SUCCESS,
};

static RNG: Mutex<SimpleCounter> = Mutex::new(SimpleCounter { state: 1 });

pub struct NtCallback {
    pub object_type: ObjectType,
    pub active: bool,
    pub callback: CallbackObject,
    callback_queue: RwLock<VecDeque<AwaitNotificationRequestAsyncCommand>>,
}

impl Hash for NtCallback {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.callback)
    }
}

unsafe impl Send for NtCallback {}
unsafe impl Sync for NtCallback {}

impl NtCallback {
    pub fn new(object_type: ObjectType) -> Self {
        Self {
            object_type,
            active: true,
            callback: RNG.lock().next_u32() as _,
            callback_queue: RwLock::new(VecDeque::new()),
        }
    }

    pub fn queue_callback_waiter(&self, wait_request: AwaitNotificationRequestAsyncCommand) {
        self.callback_queue.write().push_back(wait_request);
    }

    pub fn dequeue_callback_waiter(&self) -> Option<AwaitNotificationRequestAsyncCommand> {
        self.callback_queue.write().pop_front()
    }

    pub fn init() -> Result<(), NTSTATUS> {
        log::info!("Initializing callbacks...");
        unsafe {
            match PsSetCreateThreadNotifyRoutineEx(
                PsCreateThreadNotifyNonSystem,
                Self::thread_callback as _,
            ) {
                STATUS_SUCCESS => {}
                err => return Err(err),
            }
            match PsSetCreateProcessNotifyRoutineEx(Some(Self::process_callback), FALSE as _) {
                STATUS_SUCCESS => {}
                err => return Err(err),
            }
        }
        log::info!("Successfully initialized callbacks");
        Ok(())
    }

    // from MSDN: Don't make calls into a user mode service to validate the process, thread, or image.
    // yeah. definitely.
    unsafe extern "C" fn process_callback(
        _process: PEPROCESS,
        id: HANDLE,
        info: PPS_CREATE_NOTIFY_INFO,
    ) {
        let mut lock = ObjectTracker::get_callbacks_lock();
        let lock = unsafe { lock.get_mut_unchecked() };

        'kv: for (_, value) in lock {
            if !value.active {
                continue;
            }
            if value.object_type != ObjectType::Process(0) {
                continue;
            }

            log::trace!(
                "Total of {} waiters in queue",
                value.callback_queue.read().len()
            );

            loop {
                match value.dequeue_callback_waiter() {
                    Some(mut awaiter) => {
                        awaiter.response = AwaitNotificationResponse {
                            object_type: ObjectType::Process(id as _),
                            object_state: match info.is_null() {
                                true => ObjectState::Deleted,
                                false => ObjectState::Created,
                            },
                        }
                        .into_raw();
                        ObjectTracker::queue_command(Box::new(awaiter));
                    }
                    None => break 'kv,
                };
            }
        }
    }
    unsafe extern "C" fn thread_callback(_pid: HANDLE, tid: HANDLE, create: BOOLEAN) {
        let mut lock = ObjectTracker::get_callbacks_lock();
        let lock = unsafe { lock.get_mut_unchecked() };

        'kv: for (_, value) in lock {
            if !value.active {
                continue;
            }
            if value.object_type != ObjectType::Thread(0) {
                continue;
            }
            loop {
                match value.dequeue_callback_waiter() {
                    Some(mut awaiter) => {
                        awaiter.response = AwaitNotificationResponse {
                            object_type: ObjectType::Thread(tid as _),
                            object_state: match create {
                                1 => ObjectState::Deleted,
                                0 => ObjectState::Created,
                                _ => unreachable!(),
                            },
                        }
                        .into_raw();
                        ObjectTracker::queue_command(Box::new(awaiter));
                    }
                    None => break 'kv,
                };
            }
        }
    }
}
