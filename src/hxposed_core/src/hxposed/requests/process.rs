use crate::hxposed::call::{HypervisorCall, HypervisorResult};
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::process::{GetProcessFieldResponse, OpenProcessResponse};
use alloc::boxed::Box;
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::hxposed::requests::process::ProcessField::Protection;
use crate::services::types::process_fields::{ProcessProtection, ProcessSignatureLevels};

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OpenProcessRequest {
    pub process_id: u32,
    pub open_type: ProcessOpenType,
}

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct CloseProcessRequest {
    pub addr: u64,
    pub open_type: ProcessOpenType,
}

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct KillProcessRequest {
    pub id: u32,
    pub exit_code: u32,
}


#[derive(Default, Debug)]
#[repr(C)]
pub struct GetProcessFieldRequest {
    pub id: u32,
    pub field: ProcessField,
    /// When set to null_mut, returns the number of bytes caller requires to allocate.
    pub user_buffer: AtomicPtr<u8>,
    pub user_buffer_len: u16,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct SetProcessFieldRequest {
    pub id: u32,
    pub field: ProcessField,
    /// When set to null_mut, returns the number of bytes caller requires to allocate.
    pub user_buffer: AtomicPtr<u8>,
    pub user_buffer_len: u16,
}

impl Clone for GetProcessFieldRequest {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            field: self.field.clone(),
            user_buffer: AtomicPtr::new(self.user_buffer.load(Ordering::Relaxed)),
            user_buffer_len: self.user_buffer_len,
        }
    }
}

impl VmcallRequest for OpenProcessRequest {
    type Response = OpenProcessResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::open_process(),
            arg1: self.process_id as _,
            arg2: self.open_type.to_bits() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process_id: request.arg1 as _,
            open_type: ProcessOpenType::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for CloseProcessRequest {
    #[deprecated(note = "This request does not provide a response. Used as a dummy")]
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::close_process(),
            arg1: self.addr as _,
            arg2: self.open_type.to_bits() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1 as _,
            open_type: ProcessOpenType::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for KillProcessRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::kill_process(),
            arg1: self.id as _,
            arg2: self.exit_code as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            id: request.arg1 as _,
            exit_code: request.arg2 as _,
        }
    }
}

impl VmcallRequest for GetProcessFieldRequest {
    type Response = GetProcessFieldResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::get_process_field(),
            arg1: self.id as _,
            arg2: self.field as _,

            extended_arg1: self.user_buffer.load(Ordering::Relaxed) as _,
            extended_arg2: self.user_buffer_len as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            id: request.arg1 as _,
            field: ProcessField::from_bits(request.arg2 as _),
            user_buffer: AtomicPtr::new(request.extended_arg1 as _),
            user_buffer_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for SetProcessFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::set_process_field(),
            arg1: self.id as _,
            arg2: self.field as _,

            extended_arg1: self.user_buffer.load(Ordering::Relaxed) as _,
            extended_arg2: self.user_buffer_len as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self{
            id: request.arg1 as _,
            field: ProcessField::from_bits(request.arg2 as _),
            user_buffer: AtomicPtr::new(request.extended_arg1 as _),
            user_buffer_len: request.extended_arg2 as _,
        }
    }
}

impl SetProcessFieldRequest {
    pub(crate) fn set_protection(id: u32, new_protection: &mut ProcessProtection) -> Self {
        Self {
            id,
            field: ProcessField::Protection,
            user_buffer: AtomicPtr::new(new_protection as *mut _ as *mut u8),
            user_buffer_len: size_of::<ProcessProtection>() as _, // 1 byte
        }
    }

    pub(crate) fn set_signature_levels(id: u32, new_levels: &mut ProcessSignatureLevels) -> Self {
        Self {
            id,
            field: ProcessField::Signers,
            user_buffer: AtomicPtr::new(new_levels as *mut _ as *mut u8),
            user_buffer_len: size_of::<ProcessSignatureLevels>() as _,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum ProcessField {
    #[default]
    Unknown,
    NtPath = 1,
    Protection = 2,
    Signers = 3
}

impl ProcessField {
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            1 => Self::NtPath,
            2 => Self::Protection,
            3 => Self::Signers,
            _ => Self::Unknown,
        }
    }

    pub const fn into_bits(self) -> u16 {
        self as _
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum ProcessOpenType {
    #[default]
    #[deprecated(note = "Has no effect.")]
    Handle = 0,
    Hypervisor = 1,
}

impl ProcessOpenType {
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => Self::Handle,
            _ => Self::Hypervisor,
        }
    }

    pub const fn to_bits(self) -> u16 {
        self as u16
    }
}
