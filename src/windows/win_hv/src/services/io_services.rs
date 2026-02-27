use crate::utils::intrin::{rdmsr_failsafe, wrmsr_failsafe};
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::io::{MsrIoRequest, MsrOperation};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::io::MsrIoResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use x86::msr::{rdmsr, wrmsr};

pub fn rw_msr(request: MsrIoRequest) -> HypervisorResponse {
    match request.operation {
        MsrOperation::Read => match rdmsr_failsafe(request.msr) {
            Some(value) => MsrIoResponse { value }.into_raw(),
            None => HypervisorResponse::not_allowed(NotAllowedReason::MsrDoesntExist),
        },
        MsrOperation::Write => match wrmsr_failsafe(request.msr, request.value) {
            Some(_) => EmptyResponse::with_service(ServiceFunction::MsrIo),
            None => HypervisorResponse::not_allowed(NotAllowedReason::MsrDoesntExist),
        },
    }
}
