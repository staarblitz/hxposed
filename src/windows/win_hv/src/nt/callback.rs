use crate::services::commands::callback::AwaitNotificationRequestAsyncCommand;
use crate::utils::rng::SimpleCounter;
use alloc::collections::VecDeque;
use core::hash::{Hash, Hasher};
use hxposed_core::hxposed::{CallbackObject, ObjectType};
use spin::{Mutex, RwLock};
use wdk_sys::_PSCREATETHREADNOTIFYTYPE::PsCreateThreadNotifyNonSystem;
use wdk_sys::ntddk::{
    PsSetCreateProcessNotifyRoutineEx,
    PsSetCreateThreadNotifyRoutineEx,
};
use wdk_sys::{
    BOOLEAN, FALSE, HANDLE, NTSTATUS, PEPROCESS, PPS_CREATE_NOTIFY_INFO, STATUS_SUCCESS,
};

static RNG: Mutex<SimpleCounter> = Mutex::new(SimpleCounter { state: 1 });

pub struct NtCallback {
    pub object_type: ObjectType,
    pub active: bool,
    pub callback: CallbackObject,
    callback_queue: RwLock<
        VecDeque<AwaitNotificationRequestAsyncCommand>>,
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
        self.callback_queue.write().pop_back()
    }

    pub fn init() -> Result<(), NTSTATUS> {
        log::info!("Initializing callbacks...");
        unsafe {
            match PsSetCreateProcessNotifyRoutineEx(Some(Self::process_callback), FALSE as _) {
                STATUS_SUCCESS => {}
                err => return Err(err),
            }
            match PsSetCreateThreadNotifyRoutineEx(
                PsCreateThreadNotifyNonSystem,
                Self::thread_callback as _,
            ) {
                STATUS_SUCCESS => {}
                err => return Err(err),
            }
        }
        log::info!("Successfully initialized callbacks");
        Ok(())
    }

/*    fn send_callback(
        plugin: &Plugin,
        response: HypervisorResponse,
        mut waiter: AwaitNotificationRequestAsyncCommand,
    ) {
        let process = plugin.process.as_ref().unwrap();
        let _ctx = process.begin_context();
        waiter.complete(response);
    }*/

    unsafe extern "C" fn process_callback(
        _process: PEPROCESS,
        _id: HANDLE,
        _info: PPS_CREATE_NOTIFY_INFO,
    ) {

        // if this process is terminated, then we should replace the

        // unsafe { &mut *PLUGINS.load(Ordering::Relaxed) }
        //     .plugins
        //     .iter()
        //     .for_each(|plugin| {
        //         if plugin.process.is_none() {
        //             return;
        //         }
        //         (*plugin)
        //             .object_table
        //             .iter_callbacks()
        //             .for_each(|callback| {
        //                 loop {
        //                     let waiter = match callback.dequeue_callback_waiter() {
        //                         Some(x) => x,
        //                         None => break,
        //                     };
        //
        //                     Self::send_callback(
        //                         plugin,
        //                         AwaitNotificationResponse {
        //                             object_type: ObjectType::Process(process as _),
        //                             object_state: match info.is_null() {
        //                                 true => ObjectState::Deleted,
        //                                 false => ObjectState::Created,
        //                             },
        //                         }
        //                         .into_raw(),
        //                         waiter,
        //                     )
        //                 }
        //             });
        //     });
    }
    unsafe extern "C" fn thread_callback(_pid: HANDLE, _tid: HANDLE, _create: BOOLEAN) {

    }
}
