use crate::nt::event::NtEvent;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::process::NtProcess;
use crate::objects::{CALLER_PROCESSES, ObjectTracker};
use crate::utils::rng::SimpleCounter;
use crate::win::{Boolean, HANDLE, NtStatus, PEPROCESS, PVOID, PsSetCreateProcessNotifyRoutineEx, PsSetCreateThreadNotifyRoutineEx, CreateThreadNotifType};
use core::hash::{Hash, Hasher};
use hxposed_core::hxposed::requests::notify::ObjectState;
use hxposed_core::hxposed::responses::notify::CallbackInformation;
use hxposed_core::hxposed::responses::VmcallResponse;
use hxposed_core::hxposed::{CallbackObject, ObjectType};
use spin::Mutex;

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

    pub fn init() -> Result<(), NtStatus> {
        log::info!("Initializing callbacks...");
        unsafe {
            PsSetCreateProcessNotifyRoutineEx(Self::process_callback as _, Boolean::False).into_result()?;
            PsSetCreateThreadNotifyRoutineEx(CreateThreadNotifType::PsCreateThreadNotifyNonSystem, Self::thread_callback as _).into_result()?;
        }
        log::info!("Successfully initialized callbacks");
        Ok(())
    }

    // from MSDN: Don't make calls into a user mode service to validate the process, thread, or image.
    // yeah. definitely.
    unsafe extern "C" fn process_callback(process: PEPROCESS, id: HANDLE, info: PVOID) {
        let process = NtProcess::from_ptr_owning(process);

        // we dont do this in vmexit so we save cycles
        if info.is_null() && process.is_hx_info_present() {
            // however, when terminating, we are indeed in context of the process being terminated.
            // windows shenanigans
            process.free_hx_info();
            ObjectTracker::pop_caller_process(process.nt_process as _);
            return;
        } else if !info.is_null()
            && !process.is_hx_info_present()
            && HxGuard::is_valid_caller(process.get_path_hash())
        {
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
                if callback.object_type != ObjectType::Process(0) {continue}

                log::info!("Firing callback for: {}", callback.callback);

                let obj = ObjectType::Process(id as _).into_raw();
                let callback_info = CallbackInformation {
                    object_type: obj.0,
                    object_value: obj.1,
                    object_state: match info.is_null() {
                        true => ObjectState::Deleted,
                        false => ObjectState::Created
                    },
                };

                log::info!("Callback information: {:?}", callback_info);

                let offset = async_state.write_type(callback_info);
                async_state.write_type_no_ring(0, offset as u32);

                log::info!("Signaling event...");
                callback.event.signal();
                log::info!("Callback fired");
            }
        })
    }

    unsafe extern "C" fn thread_callback(_process_id: HANDLE, thread_id: HANDLE, create: Boolean) {
        CALLER_PROCESSES.lock().iter_mut().for_each(|nt| {
            let object_tracker = nt.get_object_tracker_unchecked();
            let async_state = nt.get_hx_async_state_unchecked();

            for (_, callback) in &mut object_tracker.callbacks {
                if callback.object_type != ObjectType::Thread(0) {continue}

                log::info!("Firing callback for: {}", callback.callback);

                let obj = ObjectType::Thread(thread_id as _).into_raw();
                let callback_info = CallbackInformation {
                    object_type: obj.0,
                    object_value: obj.1,
                    object_state: match create {
                        Boolean::False => ObjectState::Deleted,
                        Boolean::True => ObjectState::Created
                    },
                };

                log::info!("Callback information: {:?}", callback_info);

                let offset = async_state.write_type(callback_info);
                async_state.write_type_no_ring(0, offset as u32);

                log::info!("Signaling event...");
                callback.event.signal();
                log::info!("Callback fired");
            }
        })
    }
}
