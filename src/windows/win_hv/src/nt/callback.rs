use crate::nt::mm::mdl::MemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::objects::ObjectTracker;
use crate::utils::rng::SimpleCounter;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::collections::btree_map::Keys;
use alloc::vec::Vec;
use wdk_sys::_MM_PAGE_PRIORITY::HighPagePriority;
use wdk_sys::_MODE::UserMode;
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
    BOOLEAN, FALSE, HANDLE, MdlMappingNoWrite, NTSTATUS, OB_CALLBACK_REGISTRATION, PEPROCESS, PPS_CREATE_NOTIFY_INFO, PVOID, STATUS_SUCCESS
};
use crate::nt::guard::hxguard::HxGuard;
use crate::objects::async_obj::AsyncState;

static RNG: Mutex<SimpleCounter> = Mutex::new(SimpleCounter { state: 1 });

pub struct NtCallback {
    pub object_type: ObjectType,
    pub active: bool,
    pub callback: CallbackObject,
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
        _process: PEPROCESS,
        id: HANDLE,
        info: PPS_CREATE_NOTIFY_INFO,
    ) {
        let process = NtProcess::current();

        // we dont do it in vmexit so we save cycles
        if info.is_null() && process.is_hx_info_present() {
            process.free_hx_info();
            return;
        } else if !info.is_null() && !process.is_hx_info_present() && HxGuard::is_valid_caller(process.get_path_hash()) {
            process.setup_hx_info().unwrap(); // we know it's not setup.
            return;
        }
    }
    unsafe extern "C" fn thread_callback(_pid: HANDLE, tid: HANDLE, create: BOOLEAN) {

    }
}
