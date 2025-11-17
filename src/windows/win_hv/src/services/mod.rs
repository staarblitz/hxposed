use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::func::ServiceFunction;

pub mod process_services;

///
/// # Handle Process Services
///
/// Dispatches the process service request to [process_services].
///
pub fn handle_process_services(guest: &mut dyn Guest, call: HypervisorCall) {
    match call.func() {
        ServiceFunction::OpenProcess => {

        }
        _ => unreachable!()
    }
}
