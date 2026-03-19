
use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use core::arch::asm;
use core::arch::x86_64::_mm_load_si128;

#[allow(dead_code)]
pub fn vmcall_typed<R: VmcallRequest>(req: R) -> Result<R::Response, HypervisorError> {
    let raw_resp = vmcall(&mut req.into_raw());
    if raw_resp.result.error_code != 0 {
        Err(HypervisorError::from_response(&raw_resp))
    } else {
        Ok(R::Response::from_raw(raw_resp))
    }
}

pub(crate) fn vmcall(request: &mut HypervisorRequest) -> HypervisorResponse {
    let mut response = HypervisorResponse::default();
    let mut result: u64;
    let mut leaf = 0x2009u64;

    if request.call.extended_args_present() {
        unsafe {
            asm!(
            "cpuid",
            inout("r8") request.arg1 => response.arg1,
            inout("r9") request.arg2 => response.arg2,
            inout("r10") request.arg3 => response.arg3,

            in("xmm0") _mm_load_si128(&request.extended_arg1 as *const _ as _),
            in("xmm1") _mm_load_si128(&request.extended_arg2 as *const _ as _),
            in("xmm2") _mm_load_si128(&request.extended_arg3 as *const _ as _),
            in("xmm3") _mm_load_si128(&request.extended_arg4 as *const _ as _),

            inout("rsi") request.call.into_bits() => result,
            inout("rcx") leaf);
        }
    } else {
        unsafe {
            asm!(
            "cpuid",
            inout("r8") request.arg1 => response.arg1,
            inout("r9") request.arg2 => response.arg2,
            inout("r10") request.arg3 => response.arg3,
            inout("rsi") request.call.into_bits() => result,
            inout("rcx") leaf);
        }
    }

    if leaf != 0x2009 {
        response.result = HypervisorResult::from_error(HypervisorError::HvNotLoaded);
    } else {
        response.result = HypervisorResult::from_bits(result);
    }

    response
}
