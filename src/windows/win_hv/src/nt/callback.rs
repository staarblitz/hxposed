use crate::nt::event::{NtEvent, WaitStatus};
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::mm::mdl::MemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::objects::async_obj::AsyncState;
use crate::objects::{ObjectTracker, CALLER_PROCESSES};
use crate::utils::rng::SimpleCounter;
use alloc::boxed::Box;
use alloc::collections::btree_map::Keys;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use hxposed_core::hxposed::requests::notify::ObjectState;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::{CallbackObject, ObjectType};
use spin::{Mutex, RwLock};
use wdk_sys::ntddk::{
    ObRegisterCallbacks, PsLookupThreadByThreadId, PsSetCreateProcessNotifyRoutineEx,
    PsSetCreateThreadNotifyRoutineEx,
};
use wdk_sys::_MM_PAGE_PRIORITY::HighPagePriority;
use wdk_sys::_MODE::UserMode;
use wdk_sys::_PSCREATETHREADNOTIFYTYPE::PsCreateThreadNotifyNonSystem;
use wdk_sys::{
    MdlMappingNoWrite, BOOLEAN, FALSE, HANDLE, NTSTATUS, OB_CALLBACK_REGISTRATION, PEPROCESS, PPS_CREATE_NOTIFY_INFO, PVOID, STATUS_SUCCESS,
};
use hxposed_core::hxposed::responses::notify::CallbackInformation;

static RNG: Mutex<SimpleCounter> = Mutex::new(SimpleCounter { state: 1 });

pub struct NtCallback {
    pub object_type: ObjectType,
    pub active: bool,
    pub callback: CallbackObject,
    pub event: NtEvent,
}

impl Hash for NtCallback {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.callback)
    }
}

unsafe impl Send for NtCallback {}
unsafe impl Sync for NtCallback {}

impl NtCallback {
    pub fn new(object_type: ObjectType, event: NtEvent) -> Self {
        Self {
            object_type,
            active: true,
            callback: RNG.lock().next_u32() as _,
            event,
        }
    }


    pub fn init() -> Result<(), NTSTATUS> {
        log::info!("Initializing callbacks...");
        unsafe {
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
        process: PEPROCESS,
        id: HANDLE,
        info: PPS_CREATE_NOTIFY_INFO,
    ) {
        let process = NtProcess::from_ptr_owning(process);

        // we dont do this in vmexit so we save cycles
        if info.is_null() && process.is_hx_info_present() {
            // however, when terminating, we are indeed in context of the process being terminated.
            // windows shenanigans
            process.free_hx_info();
            ObjectTracker::pop_caller_process(process.nt_process as _);
            return;
        } else if !info.is_null() && !process.is_hx_info_present() && HxGuard::is_valid_caller(process.get_path_hash()) {
            // we are not in context of the process that is being created
            // we are in context of the parent
            // this wasted me 2 hours
            let _ctx = process.begin_context();
            process.setup_hx_info().unwrap(); // we know it's not setup.
            ObjectTracker::add_caller_process(process);
            return;
        }

        CALLER_PROCESSES.lock().iter_mut().for_each(|nt| {
            let object_tracker = nt.get_object_tracker_unchecked();
            let async_state = nt.get_hx_async_state_unchecked();

            for (_, callback) in &mut object_tracker.callbacks {
                let obj = ObjectType::Process(id as _).into_raw();
                let callback_info = CallbackInformation {
                    object_type: obj.0,
                    object_value: obj.1,
                    object_state: ObjectState::Created,
                };

                let offset = async_state.write_type(callback_info);
                async_state.write_type_no_ring(0, offset as u32);

                // FIXME: this may be an unnecessary practice
                // if returns true, that means the event was already signaled and we need to wait for user-mode app to finish.
                if callback.event.signal() {
                    match callback.event.wait(false, 200) {
                        WaitStatus::TimedOut => log::warn!("Timeout for waiting callback handle to get signaled. Continuing anyway..."),
                        WaitStatus::Alerted => unreachable!(),
                        WaitStatus::Signaled => {}
                    }
                    callback.event.signal();
                }
            }
        })
    }
    unsafe extern "C" fn thread_callback(_pid: HANDLE, tid: HANDLE, create: BOOLEAN) {}
}
