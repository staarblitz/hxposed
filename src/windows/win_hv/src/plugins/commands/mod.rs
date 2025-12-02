use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::responses::HypervisorResponse;

pub mod process;

pub trait AsyncCommand: Any {
    fn get_service_function(&self) -> ServiceFunction;
    fn complete(&mut self, result: HypervisorResponse);
    fn as_any(&self) -> &dyn Any;
}