use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorSource, InternalErrorCode};
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::services::async_service::AsyncInfo;
use core::arch::asm;
use core::arch::x86_64::_mm_load_si128;
use core::ops::BitAnd;
use core::sync::atomic::Ordering;

// my dear Rust, you are beautiful. but also so annoying.
fn u128_to_sliced(value: u128) -> [i64; 2] {
    [
        value.bitand(u64::MAX as u128) as u64 as i64,
        (value >> 64) as u64 as i64,
    ]
}

pub fn vmcall_typed<R: VmcallRequest>(
    req: R,
    async_info: Option<&mut AsyncInfo>,
) -> Result<R::Response, HypervisorError> {
    let raw_resp = vmcall(req.into_raw(), async_info);
    R::Response::from_raw(raw_resp)
}

pub(crate) fn vmcall(
    request: *mut HypervisorRequest,
    mut async_info: Option<&mut AsyncInfo>,
) -> HypervisorResponse {
    let mut request = unsafe { &mut *request };

    match async_info {
        Some(_) => request.call.set_is_async(true),
        None => request.call.set_is_async(false),
    }

    let mut response = HypervisorResponse::default();
    let mut result = 0;

    let handle = match async_info {
        Some(ref async_info) => async_info.handle,
        None => 0,
    };
    let shared_mem = match async_info {
        Some(ref mut async_info) => async_info.result_values.lock().as_mut_ptr(),
        None => 0 as _,
    };

    unsafe {
        // save rsi and xmm4, since they are considered non-volatile
        asm!("pinsrq xmm4, rsi, 0", "pinsrq xmm4, r12, 1")
    }

    let mut leaf = 0x2009;
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

    // that means hypervisor did not handle our cpuid trap.
    if leaf != 0x2009 {
        response.result = HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::NotLoaded)
    } else {
        response.result = HypervisorResult::from_bits(result)
    }

    response
}
