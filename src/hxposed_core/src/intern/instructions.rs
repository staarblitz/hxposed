
use crate::error::HxError;
use crate::hxposed::call::HxResult;
use crate::hxposed::requests::{HxRequest, SyscallRequest};
use crate::hxposed::responses::{HxResponse, SyscallResponse};
use core::arch::asm;
use core::arch::x86_64::_mm_load_si128;

#[allow(dead_code)]
pub fn vmcall_typed<R: SyscallRequest>(req: R) -> Result<R::Response, HxError> {
    let raw_resp = vmcall(&mut req.into_raw());
    if raw_resp.result.error_code != 0 {
        Err(HxError::from_response(&raw_resp))
    } else {
        Ok(R::Response::from_raw(raw_resp))
    }
}

pub(crate) fn vmcall(request: &mut HxRequest) -> HxResponse {
    let mut response = HxResponse::default();
    let mut result: u64;
    let mut leaf = 0x2009u64;

    if request.call.extended_args_present() {
        unsafe {
            asm!(
            "syscall",
            inout("r8") request.arg1 => response.arg1,
            inout("r9") request.arg2 => response.arg2,
            inout("r10") request.arg3 => response.arg3,

            in("xmm0") _mm_load_si128(&request.extended_arg1 as *const _ as _),
            in("xmm1") _mm_load_si128(&request.extended_arg2 as *const _ as _),
            in("xmm2") _mm_load_si128(&request.extended_arg3 as *const _ as _),
            in("xmm3") _mm_load_si128(&request.extended_arg4 as *const _ as _),

            inout("rsi") request.call.into_bits() => result,
            inout("rax") leaf);
        }
    } else {
        unsafe {
            asm!(
            "syscall",
            inout("r8") request.arg1 => response.arg1,
            inout("r9") request.arg2 => response.arg2,
            inout("r10") request.arg3 => response.arg3,
            inout("rsi") request.call.into_bits() => result,
            inout("rax") leaf);
        }
    }

    if leaf != 0x2009 {
        response.result = HxResult::from_error(HxError::HvNotLoaded);
    } else {
        response.result = HxResult::from_bits(result);
    }

    response
}
