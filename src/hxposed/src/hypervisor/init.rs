use crate::hypervisor::Hypervisor;
use alloc::vec;

static mut HYPERVISOR: Hypervisor = Hypervisor { cpus: vec![] };

#[allow(static_mut_refs)]
pub(crate) fn init_hypervisor() {
    log::info!("Initializing hypervisor");
    unsafe {
        HYPERVISOR.virtualize_system()
    }
}
