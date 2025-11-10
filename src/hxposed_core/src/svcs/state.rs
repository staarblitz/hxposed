use crate::error::HypervisorError;
use crate::hxposed::requests::status::StatusRequest;
use crate::hxposed::responses::status::StatusResponse;
use crate::intern::instructions::vmcall_typed;

#[unsafe(export_name = "get_hx_state")]
pub extern "C" fn get_hx_state(response: *mut StatusResponse) -> HypervisorError {
    match vmcall_typed(StatusRequest::default()) {
        Ok(r) => unsafe {
            *response = r;
            HypervisorError::ok()
        },
        Err(e) => e,
    }
}
