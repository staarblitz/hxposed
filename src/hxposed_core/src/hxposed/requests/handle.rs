use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::{Handle, ProcessObject};
use crate::hxposed::responses::handle::GetHandleObjectResponse;

#[derive(Clone, Default, Debug)]
pub struct UpgradeHandleRequest {
    pub handle: Handle,
    pub process: ProcessObject,
    pub access_rights: u32,
}

#[derive(Clone, Default, Debug)]
pub struct SwapHandleObjectRequest {
    pub handle: Handle,
    pub process: ProcessObject,
    pub object: u64,
}

#[derive(Clone, Default, Debug)]
pub struct GetHandleObjectRequest {
    pub handle: Handle,
    pub process: ProcessObject,
}

impl VmcallRequest for GetHandleObjectRequest {
    type Response = GetHandleObjectResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::upgrade_handle(),
            arg1: self.handle,
            arg2: self.process,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            handle: request.arg1,
            process: request.arg2,
        }
    }
}

impl VmcallRequest for UpgradeHandleRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::upgrade_handle(),
            arg1: self.handle,
            arg2: self.process,
            arg3: self.access_rights as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            handle: request.arg1,
            process: request.arg2,
            access_rights: request.arg3 as _,
        }
    }
}

impl VmcallRequest for SwapHandleObjectRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::swap_handle_obj(),
            arg1: self.handle,
            arg2: self.process,
            arg3: self.object,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            handle: request.arg1,
            process: request.arg2,
            object: request.arg3,
        }
    }
}
