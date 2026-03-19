use crate::hypervisor::init::HYPERVISOR;
use crate::hypervisor::vmfs::Registers;
use crate::utils::logger::LogEvent;
use crate::win::KeGetCurrentProcessorNumber;
use alloc::boxed::Box;
use x86::segmentation::{SegmentSelector, cs};
use crate::utils::intrin;

unsafe extern "C" {
    fn hv_int_pf();
    fn hv_int_bp();
    fn hv_int_gp();
    fn hv_int_df();
}

#[unsafe(no_mangle)]
pub extern "C" fn vm_int_handler(registers: *mut Registers, vector: u32, error: u32) {
    let vcpu = unsafe { &mut HYPERVISOR.cpus[KeGetCurrentProcessorNumber() as usize] };
    vcpu.hvfs
        .logger
        .error(LogEvent::Catastrophic(registers as _, vector, error));
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct InterruptDescriptorTableRaw([InterruptDescriptorTableEntry; 0x100]);

impl InterruptDescriptorTableRaw {
    pub fn new() -> Box<InterruptDescriptorTableRaw> {
        const BP_INDEX: usize = 0x03;
        const DF_INDEX: usize = 0x08;
        const GP_INDEX: usize = 0x0D;
        const PF_INDEX: usize = 0x0E;

        let mut idt = unsafe { Box::<InterruptDescriptorTableRaw>::new_zeroed().assume_init() };

        idt.0[BP_INDEX] = InterruptDescriptorTableEntry::new(hv_int_bp as _, cs());
        idt.0[DF_INDEX] = InterruptDescriptorTableEntry::new(hv_int_df as _, cs());
        idt.0[GP_INDEX] = InterruptDescriptorTableEntry::new(hv_int_gp as _, cs());
        idt.0[PF_INDEX] = InterruptDescriptorTableEntry::new(hv_int_pf as _, cs());

        idt
    }
}

#[derive(Debug)]
#[repr(C, align(16))]
pub struct InterruptDescriptorTableEntry {
    offset_low: u16,
    selector: u16,
    reserved_1: u8,
    gate_type: u8,
    offset_high: u16,
    offset_upper: u32,
    reserved_2: u32,
}

impl InterruptDescriptorTableEntry {
    pub fn new(handler: usize, cs: SegmentSelector) -> Self {
        const INTERRUPT_GATE: u8 = 0x8E;
        Self {
            offset_low: handler as _,
            selector: cs.bits(),
            reserved_1: 0,
            gate_type: INTERRUPT_GATE,
            offset_high: (handler >> 16) as _,
            offset_upper: (handler >> 32) as _,
            reserved_2: 0,
        }
    }
}
