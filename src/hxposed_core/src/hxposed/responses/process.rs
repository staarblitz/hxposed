use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::process::ProcessField;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone)]
pub struct GetProcessFieldResponse {
    pub field: ProcessField,
}

impl VmcallResponse for GetProcessFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            field: ProcessField::from_raw_enum(raw.arg1, raw.arg2),
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.field.clone().into_raw_enum();
        /*
        let len = unsafe {
                    *(offset as *mut u32)
                };

                let mut slice = [len; 0];
                unsafe {
                    core::ptr::copy_nonoverlapping((offset + 4) as _, slice.as_mut_ptr(), len as _);
                }


         */
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetProcessField),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}
