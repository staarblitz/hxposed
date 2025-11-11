use crate::error::HypervisorError;
use crate::hxposed::requests::auth::AuthorizationRequest;
use crate::hxposed::responses::auth::AuthorizationResponse;
use crate::intern::instructions::vmcall_typed;

#[unsafe(export_name = "hx_auth")]
pub extern "C" fn hx_auth(request: *mut AuthorizationRequest,response: *mut AuthorizationResponse) -> HypervisorError {
    let request = unsafe{&mut *(request)};
    match vmcall_typed(request.clone()) {
        Ok(r) => unsafe {
            *response = r;
            HypervisorError::ok()
        },
        Err(e) => e,
    }
}
