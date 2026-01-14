use crate::nt::callback::NtCallback;
use crate::nt::mdl::MemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::nt::token::NtToken;
use crate::services::commands::AsyncCommand;
use crate::utils::pop_guard::PopGuard;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use hashbrown::HashMap;
use hxposed_core::hxposed::{CallbackObject, MdlObject, ProcessObject, ThreadObject, TokenObject};
use spin::mutex::SpinMutex;
use spin::Once;

static GLOBAL_CALLBACKS: SpinMutex<Once<HashMap<u64, NtCallback>>> = SpinMutex::new(Once::new());
static GLOBAL_THREADS: SpinMutex<Once<HashMap<u64, NtThread>>> = SpinMutex::new(Once::new());
static GLOBAL_MDLS: SpinMutex<Once<HashMap<u64, MemoryDescriptor>>> = SpinMutex::new(Once::new());
static GLOBAL_PROCESSES: SpinMutex<Once<HashMap<u64, NtProcess>>> = SpinMutex::new(Once::new());
static GLOBAL_TOKENS: SpinMutex<Once<HashMap<u64, NtToken>>> = SpinMutex::new(Once::new());
static GLOBAL_ASYNC_COMMANDS: SpinMutex<Once<VecDeque<Box<dyn AsyncCommand>>>> =
    SpinMutex::new(Once::new());
pub struct ObjectTracker {}

impl ObjectTracker {
    pub fn init_objects() {
        GLOBAL_CALLBACKS.lock().call_once(|| HashMap::new());
        GLOBAL_THREADS.lock().call_once(|| HashMap::new());
        GLOBAL_MDLS.lock().call_once(|| HashMap::new());
        GLOBAL_PROCESSES.lock().call_once(|| HashMap::new());
        GLOBAL_TOKENS.lock().call_once(|| HashMap::new());
        GLOBAL_ASYNC_COMMANDS.lock().call_once(|| VecDeque::new());
    }

    pub fn queue_command(command: Box<dyn AsyncCommand>) {
        unsafe {
            GLOBAL_ASYNC_COMMANDS
                .lock()
                .get_mut_unchecked()
                .push_back(command);
        }
    }

    pub fn dequeue_command() -> Option<Box<dyn AsyncCommand>> {
        unsafe { GLOBAL_ASYNC_COMMANDS.lock().get_mut_unchecked().pop_front() }
    }

    pub fn add_mdl(mdl: MemoryDescriptor) {
        unsafe {
            GLOBAL_MDLS
                .lock()
                .get_mut_unchecked()
                .insert(mdl.mdl.ptr as _, mdl);
        }
    }

    pub fn add_callback(callback: NtCallback) {
        unsafe {
            GLOBAL_CALLBACKS
                .lock()
                .get_mut_unchecked()
                .insert(callback.callback, callback);
        }
    }

    pub fn add_open_process(process: NtProcess) {
        unsafe {
            GLOBAL_PROCESSES
                .lock()
                .get_mut_unchecked()
                .insert(process.nt_process as _, process);
        }
    }

    pub fn add_open_thread(thread: NtThread) {
        unsafe {
            GLOBAL_THREADS
                .lock()
                .get_mut_unchecked()
                .insert(thread.nt_thread as _, thread);
        }
    }

    pub fn add_open_token(token: NtToken) {
        unsafe {
            GLOBAL_TOKENS
                .lock()
                .get_mut_unchecked()
                .insert(token.nt_token as _, token);
        }
    }

    pub fn get_allocated_mdl(
        mdl_addr: MdlObject,
    ) -> Option<PopGuard<'static, u64, MemoryDescriptor>> {
        let mut lock = GLOBAL_MDLS.lock();
        unsafe {
            match lock.get_mut_unchecked().remove(&mdl_addr) {
                None => None,
                Some(x) => Some(PopGuard::new(x.mdl.ptr as u64, x, &GLOBAL_MDLS)),
            }
        }
    }

    pub fn get_open_thread(addr: ThreadObject) -> Option<PopGuard<'static, u64, NtThread>> {
        let mut lock = GLOBAL_THREADS.lock();
        unsafe {
            match lock.get_mut_unchecked().remove(&addr) {
                None => None,
                Some(x) => Some(PopGuard::new(x.nt_thread as u64, x, &GLOBAL_THREADS)),
            }
        }
    }

    pub fn get_open_process(addr: ProcessObject) -> Option<PopGuard<'static, u64, NtProcess>> {
        let mut lock = GLOBAL_PROCESSES.lock();
        unsafe {
            match lock.get_mut_unchecked().remove(&addr) {
                None => None,
                Some(x) => Some(PopGuard::new(x.nt_process as u64, x, &GLOBAL_PROCESSES)),
            }
        }
    }

    pub fn get_open_token(addr: TokenObject) -> Option<PopGuard<'static, u64, NtToken>> {
        let mut lock = GLOBAL_TOKENS.lock();
        unsafe {
            match lock.get_mut_unchecked().remove(&addr) {
                None => None,
                Some(x) => Some(PopGuard::new(x.nt_token as u64, x, &GLOBAL_TOKENS)),
            }
        }
    }

    pub fn get_callback(callback: CallbackObject) -> Option<PopGuard<'static, u64, NtCallback>> {
        let mut lock = GLOBAL_CALLBACKS.lock();
        unsafe {
            match lock.get_mut_unchecked().remove(&callback) {
                None => None,
                Some(x) => Some(PopGuard::new(x.callback as u64, x, &GLOBAL_CALLBACKS)),
            }
        }
    }
}
