use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorCode, ErrorSource};
use crate::hxposed::request::{HypervisorRequest, VmcallRequest};
use crate::hxposed::response::{HypervisorResponse, VmcallResponse};
use core::arch::asm;

pub fn vmcall_typed<R: VmcallRequest>(req: R) -> Result<R::Response, HypervisorError> {
    let raw_resp = vmcall(req.into_raw());
    let err = HypervisorError::from(raw_resp.result);
    if err.is_err() {
        return Err(err);
    }
    Ok(R::Response::from_raw(raw_resp))
}
pub(crate) fn vmcall(request: HypervisorRequest) -> HypervisorResponse {
    let mut response = HypervisorResponse::default();
    let mut result = 0;
    let mut leaf = 0x2009;
    unsafe {
        asm!(
        "cpuid",
        inout("r8") request.arg1 => response.arg1,
        inout("r9") request.arg2 => response.arg2,
        inout("r10") request.arg3 => response.arg3,
        inout("rsi") request.call.into_bits() => result,
        inout("rcx") leaf => leaf);
    }

    // that means hypervisor did not handle our cpuid trap.
    if leaf != 0x2009 {
        response.result = HypervisorResult::error(ErrorSource::Hv, ErrorCode::NotLoaded)
    } else {
        response.result = HypervisorResult::from_bits(result)
    }

    response
}
