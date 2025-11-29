use crate::PLUGINS;
use crate::services::process_services::kill_process_sync;
use crate::win::alloc::PoolAlloc;
use crate::win::{
    NT_PS_GET_CONTEXT_THREAD_INTERNAL, NT_PS_SET_CONTEXT_THREAD_INTERNAL, PsTerminateProcess,
    RtlCreateUserThread, ZwResumeThread, timing,
};
use core::sync::atomic::Ordering;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::KillProcessRequest;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use wdk::println;
use wdk_sys::_MODE::{KernelMode, UserMode};
use wdk_sys::ntddk::{
    KeDelayExecutionThread, KeGetCurrentIrql, ObOpenObjectByPointer, ObReferenceObjectByHandle,
    ZwClose,
};
use wdk_sys::{
    CLIENT_ID, CONTEXT, FALSE, HANDLE, LARGE_INTEGER, OBJ_KERNEL_HANDLE, PASSIVE_LEVEL, PEPROCESS,
    PETHREAD, PVOID, PsProcessType, PsThreadType,
};
use wdk_sys::{PROCESS_ALL_ACCESS, STATUS_SUCCESS, THREAD_ALL_ACCESS, TRUE};

///
/// # Async Worker Threa
///
/// Dequeues commands from each plugin's async command queue, "works" them, fires the result callback.
pub unsafe extern "C" fn async_worker_thread(_argument: PVOID) {
    let mut interval = timing::relative(timing::milliseconds(2500));
    let plugins = unsafe { &mut *PLUGINS.load(Ordering::Relaxed) };

    let irql = unsafe { KeGetCurrentIrql() };
    assert_eq!(irql, PASSIVE_LEVEL as _);

    loop {
        'inner: for plugin in plugins.plugins.iter_mut() {
            let command = match plugin.dequeue_command() {
                None => {
                    println!("Nothing on queue. Delaying execution...");
                    let _ = unsafe {
                        KeDelayExecutionThread(
                            KernelMode as _,
                            FALSE as _,
                            &mut interval as *mut _ as *mut LARGE_INTEGER, // weirdo nt api types
                        )
                    };
                    break 'inner;
                }
                Some(x) => {
                    println!("Found something on queue!");
                    x
                }
            };

            let result = match command.get_service_function() {
                ServiceFunction::KillProcess => {
                    let call = command
                        .get_call()
                        .downcast_ref::<KillProcessRequest>()
                        .unwrap();

                    Some(kill_process_sync(call, &plugin))
                }
                _ => None,
            };

            match result {
                Some(resp) => {
                    for handler in &mut plugin.handlers {
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
                        let mut cid = CLIENT_ID::default();
                        match unsafe {
                            RtlCreateUserThread(
                                handle,
                                Default::default(),
                                TRUE as _,
                                0,
                                Default::default(),
                                Default::default(),
                                handler.handler as *mut _,
                                Default::default(),
                                &mut thread_handle,
                                &mut cid,
                            )
                        } {
                            STATUS_SUCCESS => {}
                            err => {
                                println!("Could not create thread: {:x}", err);
                                let _ = unsafe { ZwClose(handle) };
                                break;
                            }
                        }

                        let mut thread_obj = PETHREAD::default();
                        match unsafe {
                            ObReferenceObjectByHandle(
                                thread_handle,
                                THREAD_ALL_ACCESS,
                                *PsThreadType,
                                KernelMode as _,
                                &mut thread_obj as *mut _ as _,
                                Default::default(),
                            )
                        } {
                            STATUS_SUCCESS => {}
                            err => {
                                println!("Could not open thread object: {:x}", err);
                                let _ = unsafe { ZwClose(handle) };
                                break;
                            }
                        }

                        // we don't need the handle since we got the real deal now
                        //unsafe {ZwClose(handle);}
                        // apparently not because microsoft are just a bunch of dicks

                        let mut ctx = CONTEXT::alloc();

                        match unsafe {
                            NT_PS_GET_CONTEXT_THREAD_INTERNAL
                                .load(Ordering::Relaxed)
                                .as_mut()
                                .unwrap()(
                                thread_obj,
                                ctx.as_mut(),
                                KernelMode as _,
                                UserMode as _,
                                KernelMode as _,
                            )
                        } {
                            STATUS_SUCCESS => {}
                            err => {
                                println!("Could not get context: {:x}", err);
                                let _ = unsafe { ZwClose(handle) };
                                break;
                            }
                        }

                        // set hypervisor return args
                        ctx.Rsi = resp.result.into_bits() as _;
                        ctx.R8 = resp.arg1;
                        ctx.R9 = resp.arg2;
                        ctx.R10 = resp.arg3;

                        // set our beloved
                        ctx.Rip = handler.handler as *mut u64 as u64;

                        match unsafe {
                            NT_PS_SET_CONTEXT_THREAD_INTERNAL
                                .load(Ordering::Relaxed)
                                .as_mut()
                                .unwrap()(
                                thread_obj,
                                ctx.as_mut(),
                                KernelMode as _,
                                UserMode as _,
                                KernelMode as _,
                            )
                        } {
                            STATUS_SUCCESS => {}
                            err => {
                                println!("Could not set context: {:x}", err);
                            }
                        }

                        match unsafe { ZwResumeThread(thread_handle, Default::default()) } {
                            STATUS_SUCCESS => {}
                            err => {
                                println!("Could not resume thread: {:x}", err); // skill issue
                            }
                        }

                        unsafe {
                            // the lion doesn't concern himself with the compiler warnings
                            ZwClose(handle);
                            ZwClose(thread_handle);
                        }
                    }
                }
                _ => {
                    // implement better error handling here
                }
            }
        }
    }
}
