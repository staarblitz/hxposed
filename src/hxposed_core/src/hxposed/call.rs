use crate::hxposed::func::ServiceFunction;
use bitfield_struct::bitfield;
use crate::error::HxError;

#[bitfield(u64)]
pub struct HxCall {
    #[bits(16)]
    pub func: ServiceFunction,
    pub ignore_result: bool,
    pub extended_args_present: bool,
    pub is_slow: bool,

    #[bits(45)]
    pub reserved: u64,
}

impl HxCall {
    pub(crate) fn get_status() -> Self {
        // For this call, other fields are ignored.
        Self::new().with_func(ServiceFunction::GetState)
    }

    pub(crate) fn get_token_obj()->Self {
        Self::new().with_func(ServiceFunction::GetHandleObject)
    }
    pub(crate) fn upgrade_handle() -> Self {
        Self::new().with_func(ServiceFunction::UpgradeHandle)
    }
    pub(crate) fn swap_handle_obj() -> Self {
        Self::new().with_func(ServiceFunction::SwapHandleObject)
    }

    pub(crate) fn exec_priv() -> Self {
        Self::new().with_func(ServiceFunction::ExecutePrivilegedInstruction)
    }

    pub(crate) fn translate_address() -> Self {
        Self::new().with_func(ServiceFunction::TranslateAddress)
    }

    pub(crate) fn msr_io() -> Self {
        Self::new().with_func(ServiceFunction::MsrIo)
    }

    pub(crate) fn set_page_attr() -> Self {
        Self::new().with_func(ServiceFunction::GetSetPageAttribute).with_extended_args_present(true)
    }

    pub(crate) fn unregister_notify_event() -> Self {
        Self::new().with_func(ServiceFunction::UnregisterNotifyEvent)
    }

    pub(crate) fn register_notify_event() -> Self {
        Self::new().with_func(ServiceFunction::RegisterNotifyEvent)
    }


    pub(crate) fn open_process() -> Self {
        Self::new().with_func(ServiceFunction::OpenProcess)
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


    pub(crate) fn close_thread() -> Self {
        Self::new().with_func(ServiceFunction::CloseThread)
    }

    pub(crate) fn open_thread() -> Self {
        Self::new().with_func(ServiceFunction::OpenThread)
    }

    pub(crate) fn describe_physical() -> Self {
        Self::new().with_func(ServiceFunction::DescribePhysicalMemory)
    }
    pub(crate) fn rmd_map() -> Self {
        Self::new().with_func(ServiceFunction::MapRawMemoryDescriptor).with_extended_args_present(true)
    }

    pub(crate) fn free_mem() -> Self {
        Self::new().with_func(ServiceFunction::FreeMemory)
    }

    pub(crate) fn mem_alloc() -> Self {
        Self::new().with_func(ServiceFunction::AllocateMemory)
    }


    pub(crate) fn get_process_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::GetProcessField)
    }

    pub(crate) fn set_process_field() -> Self {
        Self::new()
            .with_func(ServiceFunction::SetProcessField)
    }

    pub(crate) fn close_process() -> Self {
        Self::new().with_func(ServiceFunction::CloseProcess)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Debug,)]
#[repr(C)]
pub struct HxResult {
    pub error_code: u32,
    pub error_reason: u32,
}

impl HxResult {
    pub const fn ok() -> Self {
        Self {
            error_code: 0,
            error_reason: 0,
        }
    }

    pub fn from_bits(bits: u64) -> Self {
        unsafe {
            core::mem::transmute(bits)
        }
    }
    pub fn into_bits(self) -> u64 {
        unsafe {
            core::mem::transmute(self)
        }
    }

    pub const fn from_error(error: HxError) -> Self {
        match error {
            HxError::Success => Self {
                error_code: 0,
                error_reason: 0,
            },
            HxError::NotAllowed(x) => Self {
                error_code: 1,
                error_reason: x.into_bits(),
            },
            HxError::NotFound(x) => Self {
                error_code: 2,
                error_reason: x.into_bits()
            },
            HxError::InvalidParameters(x) => Self {
                error_code: 3,
                error_reason: x
            },
            HxError::NtError(x) => Self {
                error_code: 4,
                error_reason: x,
            },
            HxError::TimedOut => Self {
                error_code: 5,
                error_reason: 0,
            },
            HxError::HvNotLoaded => Self {
                error_code: 6,
                error_reason: 0
            },
            HxError::Unknown => Self {
                error_code: u32::MAX,
                error_reason: u32::MAX
            }
        }
    }
}