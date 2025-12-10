use crate::error::HypervisorError;
use crate::hxposed::requests::security::*;
use crate::hxposed::requests::Vmcall;
use crate::services::types::security_fields::*;

pub struct HxToken {
   addr: u64,
}

impl HxToken {
    pub fn from_raw_object(addr: u64) -> Result<HxToken, HypervisorError> {
        let result = OpenTokenRequest {
            addr
        }.send()?;

        Ok(Self {
            addr: result.addr
        })
    }
}