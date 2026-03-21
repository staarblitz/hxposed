use crate::hypervisor::init::HYPERVISOR;
use crate::hypervisor::vmfs::Registers;
use crate::utils::logger::LogEvent;
use crate::win::KeGetCurrentProcessorNumber;
use alloc::boxed::Box;
use bit_field::BitField;
use bitfield_struct::bitfield;
use x86::segmentation::{SegmentSelector, cs};
use crate::size_assert;
use crate::utils::intrin;

unsafe extern "C" {
    fn hv_int_pf();
    fn hv_int_bp();
    fn hv_int_gp();
    fn hv_int_df();
}

#[unsafe(no_mangle)]
pub extern "C" fn vm_int_handler(vector: u32, error: u32) {
    let vcpu = unsafe { &mut HYPERVISOR.cpus[KeGetCurrentProcessorNumber() as usize] };
    vcpu.hvfs
        .logger
        .error(LogEvent::Catastrophic(vcpu.hvfs.registers.as_ref() as *const _ as _, vector, error));
}

#[repr(C, align(4096))]
pub struct InterruptDescriptorTableRaw([InterruptDescriptorTableEntry; 0x100]);

impl InterruptDescriptorTableRaw {
    pub fn new() -> Box<InterruptDescriptorTableRaw> {
        const BP_INDEX: usize = 0x03;
        const DF_INDEX: usize = 0x08;
        const GP_INDEX: usize = 0x0D;
        const PF_INDEX: usize = 0x0E;

        let mut idt = unsafe { Box::<InterruptDescriptorTableRaw>::new_zeroed().assume_init() };

        idt.0[BP_INDEX] = InterruptDescriptorTableEntry::new(hv_int_bp as _, cs(), 1);
        idt.0[DF_INDEX] = InterruptDescriptorTableEntry::new(hv_int_df as _, cs(), 2);
        idt.0[GP_INDEX] = InterruptDescriptorTableEntry::new(hv_int_gp as _, cs(), 3);
        idt.0[PF_INDEX] = InterruptDescriptorTableEntry::new(hv_int_pf as _, cs(), 4);

        idt
    }
}

// it will be aligned on a 16-byte boundary anyway
#[repr(C, packed(1))]
pub struct InterruptStackTableRaw {
    reserved: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    reserved1: u64,
    // Rust abstractions >>>>>>>>>>>>>>
    pub ist1: u64,
    pub ist2: u64,
    pub ist3:u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    reserved2: u64,
    reserved3: u16,
    pub iopb: u16
}
size_assert!(InterruptStackTableRaw, 104);

impl InterruptStackTableRaw {
    pub fn new() -> Box<InterruptStackTableRaw> {
        let mut ist = unsafe { Box::<InterruptStackTableRaw>::new_zeroed().assume_init() };
        ist.iopb = size_of::<InterruptStackTableRaw>() as _; // needed if we don't plan to use io bitmaps (we dont)

        unsafe {
            ist.ist1 = Box::leak(Box::new([0u8; 1024])).as_ptr().byte_add(1024) as _;
            ist.ist2 = Box::leak(Box::new([0u8; 1024])).as_ptr().byte_add(1024) as _;
            ist.ist3 = Box::leak(Box::new([0u8; 1024])).as_ptr().byte_add(1024) as _;
            ist.ist4 = Box::leak(Box::new([0u8; 1024])).as_ptr().byte_add(1024) as _;
        }
        ist
    }
}

// these stay... for now
#[repr(C, align(16))]
pub struct GlobalDescriptorTableEntry {
    limit: u16,
    base_low: u16,
    base_mid: u8,
    access_byte: u8,
    limit_flags: u8, // could have used LimitFlags, but then it would not "fit"
    base_upper: u8,
    base_high: u32,
    /// For system segments
    base_max: u32, // iphone 27 ultra pro upper max
    reserved: u32,
}

impl GlobalDescriptorTableEntry {
    fn new(base: u64, limit_low: u16, limit_flags: u8, access_byte: u8) -> Self {
        Self {
            limit: limit_low,
            base_low: base.get_bits(0..16) as _,
            base_mid: base.get_bits(16..24) as _,
            base_upper: base.get_bits(24..32) as _,
            base_high: base.get_bits(32..64) as _,

            access_byte,
            limit_flags,

            base_max: 0,
            reserved: 0,
        }
    }
}

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
    pub fn new(handler: usize, cs: SegmentSelector, ist_index: u8) -> Self {
        const INTERRUPT_GATE: u8 = 0x8E;
        let ist_index = ist_index | 0x3;
        Self {
            offset_low: handler as _,
            selector: cs.bits(),
            reserved_1: ist_index,
            gate_type: INTERRUPT_GATE,
            offset_high: (handler >> 16) as _,
            offset_upper: (handler >> 32) as _,
            reserved_2: 0,
        }
    }
}


#[bitfield(u8)]
pub struct NormalAccessByte {
    pub accessed: bool,
    pub rw: bool,
    pub dc: bool,
    pub executable: bool,
    pub descriptor_type: bool,
    #[bits(2)]
    pub dpl: u8,
    pub present: bool,
}
#[bitfield(u8)]
pub struct SystemAccessByte {
    #[bits(4)]
    pub segment_type: u8,
    pub descriptor_type: bool,
    #[bits(2)]
    pub dpl: u8,
    pub present: bool,
}

impl SystemAccessByte {
    pub const SEGMENT_TYPE_LDT: u8 = 0x2;
    pub const SEGMENT_TYPE_TSS_AVAILABLE: u8 = 0x9;
    pub const SEGMENT_TYPE_TSS_BUSY: u8 = 0xB;
}

#[bitfield(u8)]
pub struct LimitFlags {
    #[bits(4)]
    pub limit: u8,
    reserved: bool,
    pub long: bool,
    pub size: bool,
    pub granularity: bool
}