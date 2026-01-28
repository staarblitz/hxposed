use crate::utils::intrin::{rdmsr_failsafe, wrmsr_failsafe};
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::io::{MsrIoRequest, MsrOperation};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::io::MsrIoResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

pub fn rw_msr(request: MsrIoRequest) -> HypervisorResponse {
    match request.operation {
        MsrOperation::Read => match rdmsr_failsafe(request.msr) {
            Ok(value) => MsrIoResponse { value }.into_raw(),
            Err(_) => HypervisorResponse::not_allowed(NotAllowedReason::MsrDoesntExist),
        },
        MsrOperation::Write => match wrmsr_failsafe(request.msr, request.value) {
            Ok(_) => EmptyResponse::with_service(ServiceFunction::MsrIo),
            Err(_) => HypervisorResponse::not_allowed(NotAllowedReason::MsrDoesntExist),
        },
    }
}
