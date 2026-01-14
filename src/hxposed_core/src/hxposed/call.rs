use crate::hxposed::error::{ErrorSource, InternalErrorCode};
use crate::hxposed::func::ServiceFunction;
use bitfield_struct::bitfield;

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HypervisorCall {
    #[bits(16)]
    pub func: ServiceFunction,
    pub is_fast: bool,
    pub ignore_result: bool,
    pub buffer_by_user: bool,
    pub yield_execution: bool,
    pub is_async: bool,
    pub extended_args_present: bool,

    #[bits(10)]
    pub reserved: u64,
}

impl HypervisorCall {
    pub(crate) fn get_status() -> Self {
        // For this call, other fields are ignored.
        Self::new().with_func(ServiceFunction::GetState)
    }

    pub(crate) fn auth() -> Self {
        Self::new().with_func(ServiceFunction::Authorize)
    }

    pub(crate) fn unregister_notify_event() -> Self {
        Self::new().with_func(ServiceFunction::UnregisterNotifyEvent)
    }

    pub(crate) fn register_notify_event() -> Self {
        Self::new().with_func(ServiceFunction::RegisterNotifyEvent)
    }

    pub(crate) fn await_notify_event() -> Self {
        Self::new().with_func(ServiceFunction::AwaitNotifyEvent)
    }

    pub(crate) fn open_process() -> Self {
        Self::new().with_func(ServiceFunction::OpenProcess)
    }

    pub(crate) fn kill_process() -> Self {
        Self::new().with_func(ServiceFunction::KillProcess)
    }

    pub(crate) fn close_token() -> Self {
        Self::new().with_func(ServiceFunction::CloseToken)
    }

    pub(crate) fn get_token_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::GetTokenField)
            .with_extended_args_present(true)
    }

    pub(crate) fn set_token_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::SetTokenField)
            .with_extended_args_present(true)
    }

    pub(crate) fn open_token() -> Self {
        Self::new().with_func(ServiceFunction::OpenToken)
    }

    pub(crate) fn get_thread_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::GetThreadField)
            .with_extended_args_present(true)
    }

    pub(crate) fn set_thread_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::SetThreadField)
            .with_extended_args_present(true)
    }

    pub(crate) fn get_set_thread_context() -> Self {
        Self::new()
            .with_func(ServiceFunction::GetSetThreadContext)
            .with_extended_args_present(true)
    }

    pub(crate) fn kill_thread() -> Self {
        Self::new().with_func(ServiceFunction::KillThread)
    }

    pub(crate) fn suspend_resume_thread() -> Self {
        Self::new().with_func(ServiceFunction::SuspendResumeThread)
    }

    pub(crate) fn close_thread() -> Self {
        Self::new().with_func(ServiceFunction::CloseThread)
    }

    pub(crate) fn open_thread() -> Self {
        Self::new().with_func(ServiceFunction::OpenThread)
    }

    pub(crate) fn get_process_threads() -> Self {
        Self::new().with_func(ServiceFunction::GetProcessThreads)
    }

    pub(crate) fn mem_map() -> Self {
        Self::new().with_func(ServiceFunction::MapMemory).with_extended_args_present(true)
    }

    pub(crate) fn free_mem() -> Self {
        Self::new().with_func(ServiceFunction::FreeMemory)
    }

    pub(crate) fn mem_alloc() -> Self {
        Self::new().with_func(ServiceFunction::AllocateMemory)
    }

    pub(crate) fn process_vm_protect() -> Self {
        Self::new().with_func(ServiceFunction::ProtectProcessMemory)
    }

    pub(crate) fn process_vm_op() -> Self {
        Self::new()
            .with_func(ServiceFunction::ProcessVMOperation)
            .with_extended_args_present(true)
    }

    pub(crate) fn get_process_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::GetProcessField)
            .with_extended_args_present(true)
    }

    pub(crate) fn set_process_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::SetProcessField)
            .with_extended_args_present(true)
    }

    pub(crate) fn close_process() -> Self {
        Self::new().with_func(ServiceFunction::CloseProcess)
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HypervisorResult {
    #[bits(16)]
    pub func: ServiceFunction,
    #[bits(2)]
    pub error_source: ErrorSource,
    #[bits(3)]
    pub error_code: InternalErrorCode,
    #[bits(11)]
    pub reserved: u16,
}

impl HypervisorResult {
    pub fn is_error(&self) -> bool {
        !(self.error_source() == ErrorSource::Hx && self.error_code() == InternalErrorCode::Ok)
    }

    pub fn ok(func: ServiceFunction) -> Self {
        Self::error(ErrorSource::Hx, InternalErrorCode::Ok).with_func(func)
    }

    pub fn error(error_source: ErrorSource, error_code: InternalErrorCode) -> Self {
        Self::error_with_cookie(error_source, error_code)
    }

    pub fn error_with_cookie(
        error_source: ErrorSource,
        error_code: InternalErrorCode,
    ) -> Self {
        HypervisorResult::default()
            .with_error_source(error_source)
            .with_error_code(error_code)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum ServiceParameter {
    #[default]
    None = 0,
    Function,
    IsFast,
    IgnoreResult,
    BufferByUser,
    YieldExecution,
    IsAsync,
    AsyncCookie,
    Arg1,
    Arg2,
    Arg3,
}

impl ServiceParameter {
    pub const fn into_bits(self) -> u32 {
        self as _
    }

    pub const fn from_bits(value: u32) -> Self {
        match value {
            1 => Self::Function,
            2 => Self::IsFast,
            3 => Self::IgnoreResult,
            4 => Self::BufferByUser,
            5 => Self::YieldExecution,
            6 => Self::IsAsync,
            7 => Self::AsyncCookie,
            8 => Self::Arg1,
            9 => Self::Arg2,
            10 => Self::Arg3,
            _ => Self::None,
        }
    }
}
