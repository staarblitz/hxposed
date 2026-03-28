use crate::hxposed::call::HxResult;
use crate::hxposed::requests::process::ProcessField;
use crate::hxposed::responses::{HxResponse, SyscallResponse};

#[derive(Clone)]
pub struct GetProcessFieldResponse {
    pub field: ProcessField,
}

impl SyscallResponse for GetProcessFieldResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            field: ProcessField::from_raw_enum(raw.arg1, raw.arg2),
        }
    }

    fn into_raw(self) -> HxResponse {
        let args = self.field.clone().into_raw_enum();
        HxResponse {
            result: HxResult::ok(),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}
