use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::KillProcessRequest;
use wdk_sys::PEPROCESS;


pub trait AsyncCommand {
    fn get_service_function(&self) -> ServiceFunction;
    fn get_call(&self) -> &dyn Any;
    fn get_inner(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct KillProcessAsyncCommand {
    pub call: KillProcessRequest,
    pub process: PEPROCESS,
}

impl AsyncCommand for KillProcessAsyncCommand {

    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::KillProcess
    }
    fn get_call(&self) -> &dyn Any {
        &self.call
    }

    fn get_inner(&self) -> &dyn Any {
        &self.process
    }
}
