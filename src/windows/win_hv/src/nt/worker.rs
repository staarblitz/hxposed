use crate::PLUGINS;
use crate::plugins::async_command::AsyncCommand;
use crate::win::{
    PsTerminateProcess, RtlCreateUserThread,
};
use core::sync::atomic::Ordering;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::KillProcessRequest;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use wdk::println;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{ObOpenObjectByPointer, ZwClose};
use wdk_sys::{HANDLE, NTSTATUS, OBJ_KERNEL_HANDLE, PEPROCESS, PVOID, PsProcessType};
use wdk_sys::{PROCESS_ALL_ACCESS, STATUS_SUCCESS, THREAD_ALL_ACCESS, TRUE};

pub unsafe extern "C" fn async_worker_thread(_argument: PVOID) {
    let plugins = unsafe { &mut *PLUGINS.load(Ordering::Relaxed) };
    for plugin in plugins.plugins.iter_mut() {
        let command = match plugin.awaiting_commands.pop_back() {
            None => {
                continue;
            }
            Some(x) => x,
        };

        let result = match command.get_service_function() {
            ServiceFunction::KillProcess => {
                let call = command
                    .get_call()
                    .downcast_ref::<KillProcessRequest>()
                    .unwrap();
                let command = command.get_inner().downcast_ref::<PEPROCESS>().unwrap();

                Some(
                    match unsafe { PsTerminateProcess(command, call.exit_code as _) } {
                        STATUS_SUCCESS => EmptyResponse::with_service(ServiceFunction::KillProcess),
                        err => HypervisorResponse::nt_error(err as _),
                    },
                )
            }
            _ => None,
        };

        match result {
            Some(resp) => {
                for handler in &plugin.handlers {
                    let mut handle = HANDLE::default();
                    match unsafe {
                        ObOpenObjectByPointer(
                            plugin.process.load(Ordering::Relaxed) as _,
                            OBJ_KERNEL_HANDLE,
                            Default::default(),
                            PROCESS_ALL_ACCESS,
                            *PsProcessType,
                            KernelMode as _,
                            &mut handle,
                        )
                    } {
                        STATUS_SUCCESS => handle,
                        err => {
                            println!("Could not open integrated process: {:x}", err);
                            break;
                        }
                    };

                    let mut thread_handle = HANDLE::default();
                    match unsafe {
                        RtlCreateUserThread(
                            handle,
                            Default::default(),
                            TRUE as _,
                            0,
                            0,
                            0,
                            handler.handler as *mut _,
                            Default::default(),
                            Default::default(),
                            Default::default(),
                        )
                    } {
                        STATUS_SUCCESS => {}
                        err => {
                            println!("Could not create thread: {:x}", err);
                            let _ = unsafe { ZwClose(handle) };
                            break;
                        }
                    }
                }
            }
            _ => {
                // implement better error handling here
            }
        }
    }
}
