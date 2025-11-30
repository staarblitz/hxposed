use core::any::Any;
use core::sync::atomic::Ordering;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::KillProcessRequest;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::AsyncInfo;
use wdk::println;
use wdk_sys::ntddk::{ProbeForWrite, ZwSetEvent};
use wdk_sys::{PEPROCESS, STATUS_SUCCESS};
use crate::nt::context::ApcProcessContext;
use crate::plugins::plugin::Plugin;

pub trait AsyncCommand: Any {
    fn get_service_function(&self) -> ServiceFunction;
    fn complete(&mut self, result: HypervisorResponse);
    fn as_any(&self) -> &dyn Any;
}

pub struct KillProcessAsyncCommand {
    pub exit_code: u32,
    pub process: PEPROCESS,
    pub info: AsyncInfo,
    pub plugin_process: PEPROCESS
}

impl KillProcessAsyncCommand {
    pub fn new(plugin_process: PEPROCESS, exit_code: u32, process: PEPROCESS, async_info: &AsyncInfo) -> Self {
        Self {
            exit_code,
            process,
            plugin_process,
            info: async_info.clone(),
        }
    }
}

impl AsyncCommand for KillProcessAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::KillProcess
    }
    fn complete(&mut self, result: HypervisorResponse) {
        // we are in context of system. we need to switch to context of plugin's process to get access to its handle table and virtual address space.
        let _ctx = ApcProcessContext::begin(self.plugin_process);

        match microseh::try_seh(|| unsafe {
            ProbeForWrite(self.info.result_values.load(Ordering::Relaxed) as _, 16, 1)
        }) {
            Ok(_) => {
                let ptr = &mut unsafe { *self.info.result_values.load(Ordering::Relaxed) };
                ptr[0] = result.result.into_bits() as u64;
                ptr[1] = result.arg1;
                ptr[2] = result.arg2;
                ptr[3] = result.arg3;
            }
            Err(x) => {
                println!("Failed to write to user buffer: {}", x);
            }
        }

        match unsafe { ZwSetEvent(self.info.handle as _, Default::default()) } {
            STATUS_SUCCESS => {}
            err => {
                println!("Failed to set event: {}", err);
            }
        }

        drop(_ctx);
        // seems like our user mode app tried to be a little too smart.
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
