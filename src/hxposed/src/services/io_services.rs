use crate::utils::intrin::{rdmsr_failsafe, wrmsr_failsafe};
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::requests::io::{MsrIoRequest, MsrOperation};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::io::MsrIoResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

pub fn rw_msr(request: MsrIoRequest) -> HypervisorResponse {
    match request.operation {
        MsrOperation::Read => match rdmsr_failsafe(request.msr) {
            Some(value) => MsrIoResponse { value }.into_raw(),
            None => HypervisorResponse::not_allowed(NotAllowedReason::AccessViolation),
        },
        MsrOperation::Write => match wrmsr_failsafe(request.msr, request.value) {
            Some(_) => EmptyResponse::default(),
            None => HypervisorResponse::not_allowed(NotAllowedReason::AccessViolation),
        },
    }
}
