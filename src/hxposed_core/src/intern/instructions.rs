use alloc::sync::Arc;

use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorSource, InternalErrorCode};
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use core::arch::asm;
use core::arch::x86_64::_mm_load_si128;
use crate::events::AsyncInfo;

#[allow(dead_code)]
pub fn vmcall_typed<R: VmcallRequest>(
    req: R,
    async_info: Option<Arc<AsyncInfo>>,
) -> Result<R::Response, HypervisorError> {
    let raw_resp = vmcall(&mut req.into_raw(), async_info);
    if raw_resp.result.is_error() {
        Err(HypervisorError::from_response(raw_resp))
    } else {
        Ok(R::Response::from_raw(raw_resp))
    }
}

pub(crate) fn vmcall(
    request: &mut HypervisorRequest,
    async_info: Option<Arc<AsyncInfo>>,
) -> HypervisorResponse {
    // SAFETY:we know it's a valid pointer.

    let (handle, shared_mem, is_async) = match async_info {
        Some(info) => (info.handle, info.result_values.lock().as_mut_ptr(), true),
        None => (0, core::ptr::null_mut(), false),
    };

    request.call.set_is_async(is_async);

    let mut response = HypervisorResponse::default();
    let mut result: u32;
    let mut leaf = 0x2009u64;

    unsafe {
        // save rsi and xmm4, since they are considered non-volatile
        // must be done in seperate because will interfere with "inouts"
        asm!("pinsrq xmm4, rsi, 0", "pinsrq xmm4, r12, 1")
    }

    if request.call.extended_args_present() {
        unsafe {
            asm!(
            "cpuid",
            inout("r8") request.arg1 => response.arg1,
            inout("r9") request.arg2 => response.arg2,
            inout("r10") request.arg3 => response.arg3,
            in("r11") handle,
            in("r12") shared_mem,

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
            in("r11") handle,
            in("r12") shared_mem,
            inout("rsi") request.call.into_bits() => result,
            inout("rcx") leaf);
        }
    }

    unsafe {
        // fetch them back from xmm4
        asm!("pextrq rsi, xmm4, 0", "pextrq r12, xmm4, 1")
    }


    if leaf != 0x2009 {
        response.result = HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::NotLoaded);
    } else {
        response.result = HypervisorResult::from_bits(result);
    }

    response
}
