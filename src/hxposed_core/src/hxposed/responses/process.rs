use crate::hxposed::call::HypervisorResult;
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
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}
