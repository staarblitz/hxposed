use crate::hxposed::{call::HypervisorCall, requests::{HypervisorRequest, VmcallRequest}, responses::empty::EmptyResponse, AsyncCookie};

#[derive(Default, Debug)]
pub struct CancelAsyncCallRequest {
    pub cookie: AsyncCookie,
}

impl VmcallRequest for CancelAsyncCallRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::cancel_async_call(),
            arg1: self.cookie,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            cookie: request.arg1
        }
    }
}