use crate::hxposed::call::{AsyncCookie, HypervisorCall};
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct AddAsyncHandlerRequest {
    pub addr: u64,
    pub cookie: AsyncCookie,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct RemoveAsyncHandlerRequest {
    pub addr: u64,
    pub cookie: AsyncCookie
}

impl VmcallRequest for RemoveAsyncHandlerRequest  {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest{
            call: HypervisorCall::remove_async_handler(),
            arg1: self.addr,
            ..Default::default()
        }
    }

    fn from_raw(call: HypervisorCall, args: (u64, u64, u64)) -> Self {
        Self {
            addr: args.0,
            cookie: call.async_cookie()
        }
    }
}

impl VmcallRequest for AddAsyncHandlerRequest  {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest{
            call: HypervisorCall::add_async_handler(),
            arg1: self.addr,
            ..Default::default()
        }
    }

    fn from_raw(call: HypervisorCall, args: (u64, u64, u64)) -> Self {
        Self {
            addr: args.0,
            cookie: call.async_cookie()
        }
    }
}