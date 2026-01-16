use crate::nt::process::NtProcess;
use crate::objects::ObjectTracker;
use crate::services::commands::memory::*;
use crate::services::commands::process::*;
use crate::services::commands::security::*;
use crate::services::commands::thread::*;
use crate::services::memory_services::*;
use crate::services::process_services::*;
use crate::services::security_services::*;
use crate::services::thread_services::*;
use crate::utils::timing;
use crate::win::KeGetCurrentThread;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::ZwWaitForSingleObject;
use wdk_sys::ntddk::{KeDelayExecutionThread, KeSetPriorityThread};
use wdk_sys::{FALSE, LARGE_INTEGER, LOW_REALTIME_PRIORITY, PVOID};
/// # Async Worker Thread
///
/// Dequeues commands from each plugin's async command queue, "works" them, fires the result callback.
pub unsafe extern "C" fn async_worker_thread(_argument: PVOID) {
    'begin: loop {
        let mut command = match ObjectTracker::dequeue_command() {
            None => {
                match unsafe {
                    ZwWaitForSingleObject(
                        ObjectTracker::get_async_event() as _,
                        FALSE as _,
                        Default::default(),
                    )
                } {
                    STATUS_SUCCESS => {}
                    _ => {}
                }

                continue 'begin;
            }
            Some(x) => x,
        };

        log::trace!(
            "Found {:?} on queue. Processing....",
            command.get_service_function()
        );

        let result = {
            let ctx = NtProcess::from_ptr(command.get_async_info().process as _).begin_context();
            match command.get_service_function() {
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
                    command
                        .as_any()
                        .downcast_ref::<SuspendResumeThreadAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::KillThread => kill_thread_sync(
                    command
                        .as_any()
                        .downcast_ref::<KillThreadAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::OpenToken => open_token_sync(
                    command
                        .as_any()
                        .downcast_ref::<OpenTokenAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::GetThreadField => get_thread_field_sync(
                    command
                        .as_any()
                        .downcast_ref::<GetThreadFieldAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::SetThreadField => set_thread_field_sync(
                    command
                        .as_any()
                        .downcast_ref::<SetThreadFieldAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::GetTokenField => get_token_field_sync(
                    command
                        .as_any()
                        .downcast_ref::<GetTokenFieldAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::SetTokenField => set_token_field_sync(
                    command
                        .as_any()
                        .downcast_ref::<SetTokenFieldAsyncCommand>()
                        .unwrap(),
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
                ServiceFunction::MapMemory => {
                    drop(ctx); // map_mdl has its own context
                    map_mdl_sync(
                        command
                            .as_any()
                            .downcast_ref::<MapMemoryAsyncCommand>()
                            .unwrap(),
                    )
                }
                ServiceFunction::FreeMemory => free_mdl_sync(
                    command
                        .as_any()
                        .downcast_ref::<FreeMemoryAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::OpenProcess => open_process_sync(
                    command
                        .as_any()
                        .downcast_ref::<OpenProcessAsyncCommand>()
                        .unwrap(),
                ),
                ServiceFunction::AwaitNotifyEvent => {
                    EmptyResponse::with_service(ServiceFunction::AwaitNotifyEvent)
                }
                _ => unreachable!("Forgot to implement this one!"),
            }
        };

        log::trace!("Work completed: {:?}", result);
        log::trace!("Signaling completion");

        let ctx = NtProcess::from_ptr(command.get_async_info().process as _).begin_context();
        command.complete(result);
        drop(ctx)
    }
}
