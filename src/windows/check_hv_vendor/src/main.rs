use std::arch::asm;
use hxposed_core::hxposed::request::VmcallRequest;
use hxposed_core::hxposed::requests::status::StatusRequest;

fn main() {
    unsafe{
        asm!("mov rcx, 0x2009", "cpuid");
    }

    println!("Preparing status request...");
    let req = StatusRequest::default();
    println!("Status request: {:?}", req);
    let response = req.send();
    println!(
        "Hypervisor status: Current: {}, Version: {}",
        response.state, response.version
    );
}
