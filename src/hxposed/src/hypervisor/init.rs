use crate::hypervisor::Hypervisor;
use alloc::vec;

#[unsafe(no_mangle)]
pub static mut HYPERVISOR: Hypervisor = Hypervisor { cpus: vec![] };

#[allow(static_mut_refs)]
pub(crate) fn init_hypervisor() {
    unsafe {
        HYPERVISOR.virtualize_system()
    }
}
