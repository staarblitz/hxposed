use crate::PLUGINS;
use crate::plugins::async_command::KillProcessAsyncCommand;
use crate::services::process_services::kill_process_sync;
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
    let mut interval = timing::relative(timing::milliseconds(2500));
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

            command.complete(match command.get_service_function() {
                ServiceFunction::KillProcess => kill_process_sync(
                    command
                        .as_any()
                        .downcast_ref::<KillProcessAsyncCommand>()
                        .unwrap(),
                    plugin,
                ),
                _ => unreachable!(),
            });
        }
    }
}
