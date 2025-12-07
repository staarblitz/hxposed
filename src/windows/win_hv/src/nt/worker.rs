use crate::PLUGINS;
use crate::nt::context::ApcProcessContext;
use crate::plugins::commands::memory::*;
use crate::plugins::commands::process::*;
use crate::plugins::commands::thread::*;
use crate::services::memory_services::*;
use crate::services::process_services::*;
use crate::services::thread_services::*;
use crate::win::timing;
use core::sync::atomic::Ordering;
use hxposed_core::hxposed::func::ServiceFunction;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::KeDelayExecutionThread;
use wdk_sys::{FALSE, LARGE_INTEGER, PVOID};

///
/// # Async Worker Thread
///
/// Dequeues commands from each plugin's async command queue, "works" them, fires the result callback.
pub unsafe extern "C" fn async_worker_thread(_argument: PVOID) {
    let mut interval = timing::relative(timing::milliseconds(100));
    let plugins = unsafe { &mut *PLUGINS.load(Ordering::Relaxed) };

    loop {
        // this labeled loops are fire 🔥🔥🔥
        'inner: for plugin in plugins.plugins.iter_mut() {
            let mut command = match plugin.dequeue_command() {
                None => {
                    let _ = unsafe {
                        KeDelayExecutionThread(
                            KernelMode as _,
                            FALSE as _,
                            &mut interval as *mut _ as *mut LARGE_INTEGER, // weirdo nt api types
                        )
                    };
                    break 'inner;
                }
                Some(x) => x,
            };

            let ctx = ApcProcessContext::begin(plugin.process);

            log::trace!("Found work on queue. Processing....");

            let result = match command.get_service_function() {
                ServiceFunction::KillProcess => kill_process_sync(
                    command
                        .as_any()
                        .downcast_ref::<KillProcessAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::GetProcessField => get_process_field_sync(
                    command
                        .as_any()
                        .downcast_ref::<GetProcessFieldAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::SetProcessField => set_process_field_sync(
                    command
                        .as_any()
                        .downcast_ref::<SetProcessFieldAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::GetProcessThreads => get_process_threads_sync(
                    command
                        .as_any()
                        .downcast_ref::<GetProcessThreadsAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::SuspendResumeThread => suspend_resume_thread_sync(
                    command.as_any().downcast_ref::<SuspendResumeThreadAsyncCommand>().unwrap()
                ),
                ServiceFunction::KillThread => kill_thread_sync(
                    command.as_any().downcast_ref::<KillThreadAsyncCommand>().unwrap()
                ),
                ServiceFunction::ProcessVMOperation => process_vm_operation_sync(
                    command
                        .as_any()
                        .downcast_ref::<RWProcessMemoryAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::ProtectProcessMemory => protect_vm_sync(
                    command
                        .as_any()
                        .downcast_ref::<ProtectProcessMemoryAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::AllocateMemory => allocate_mdl_sync(
                    command
                        .as_any()
                        .downcast_ref::<AllocateMemoryAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::MapMemory => map_mdl_sync(
                    command
                        .as_any()
                        .downcast_ref::<MapMemoryAsyncCommand>()
                        .unwrap(),
                ),
                _ => unreachable!("Forgot to implement this one!"),
            };

            log::trace!("Work completed: {:?}", result);
            log::trace!("Signaling completion");
            command.complete(result);

            drop(ctx);
        }
    }
}
