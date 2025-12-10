use alloc::boxed::Box;
use core::mem;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::OpenObjectResponse;

pub struct OpenTokenRequest {
    pub addr: u64
}

impl VmcallRequest for OpenTokenRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::open_token(),
            arg1: self.addr,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1
        }
    }
}