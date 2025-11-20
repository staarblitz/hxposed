use crate::error::HypervisorError;
use crate::hxposed::call::{AsyncCookie, HypervisorCall};
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::vmcall;
use crate::services::async_service::AsyncNotifyHandler;

pub mod auth;
pub mod process;
pub mod status;
pub mod async_help;

#[derive(Copy, Clone, Default, Debug)]
pub struct HypervisorRequest {
    pub(crate) call: HypervisorCall,
    pub(crate) arg1: u64,
    pub(crate) arg2: u64,
    pub(crate) arg3: u64,
}

pub trait VmcallRequest {
    type Response: VmcallResponse;
    fn into_raw(self) -> HypervisorRequest;
    fn from_raw(call: HypervisorCall, args: (u64, u64, u64)) -> Self;
}

pub trait Vmcall<T: VmcallRequest> {
    fn send(self) -> Result<T::Response, HypervisorError>;
    fn send_async(self, handler: &mut AsyncNotifyHandler);
}

impl<T> Vmcall<T> for T
where
    T: VmcallRequest,
{
    fn send(self) -> Result<T::Response, HypervisorError> {
        T::Response::from_raw(vmcall(self.into_raw()))
    }

    async fn send_async(self, handler: &mut AsyncNotifyHandler) {
        let mut raw = self.into_raw();
        raw.call.set_is_async(true);
        raw.call.set_async_cookie(handler.cookie);

        vmcall(raw);
    }
}
