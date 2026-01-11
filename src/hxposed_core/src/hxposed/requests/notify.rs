use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::notify::NotifyEventResponse;

pub struct AwaitNotifyEventRequest {}

impl VmcallRequest for AwaitNotifyEventRequest {
    type Response = NotifyEventResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::await_notify_event(),
            ..Default::default()
        }
    }

    fn from_raw(_request: &HypervisorRequest) -> Self {
        todo!()
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ObjectEventType {
    Created,
    Modified,
    Deleted,
}

impl ObjectEventType {
    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Created,
            1 => Self::Modified,
            2 => Self::Deleted,
            _ => unreachable!(),
        }
    }

    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
