//! This module implements architecture agnostic parts of the host code.

use core::arch::asm;
use bit_field::BitField;
use hxposed_core::hxposed::call::{HypervisorCall};
use x86::{
    controlregs::{Cr4, Xcr0},
    cpuid::cpuid,
};
use x86::bits64::rflags::RFlags;
use hxposed_core::hxposed::responses::HypervisorResponse;
use super::{amd::Amd, intel::Intel};
use crate::hypervisor::{
    HV_CPUID_INTERFACE, HV_CPUID_VENDOR_AND_MAX_FUNCTIONS, SHARED_HOST_DATA, apic_id,
    registers::Registers,
    x86_instructions::{cr4, cr4_write, rdmsr, wrmsr, xsetbv},
};

/// The entry point of the hypervisor.
pub(crate) fn main(registers: &Registers) -> ! {
    // Disable interrupt for a couple of reasons. (1) to avoid panic due to
    // interrupt, and (2) to avoid inconsistent guest initial state.
    //
    // (1): In this path, we will switch to the host IDT if specified. The host
    // IDT only panics on any interrupt. This is an issue on UEFI where we update
    // the IDT.
    // (2): An interrupt may change the system register values before and after,
    // which could leave the guest initial state inconsistent because we copy the
    // current system register values one by one for the guest. For example, we
    // set a SS value as non-zero for the guest, interrupt occurs and SS becomes
    // zero, then we set SS access rights for the guest based on SS being zero.
    // That would leave the guest SS and SS access rights inconsistent. This is
    // an issue on Windows.
    //
    // Note that NMI is still possible and can cause the same issue. We just
    // never observed it causing the described issues.
    unsafe { x86::irq::disable() };

    // Start the host on the current processor.
    if x86::cpuid::CpuId::new().get_vendor_info().unwrap().as_str() == "GenuineIntel" {
        virtualize_core::<Intel>(registers)
    } else {
        virtualize_core::<Amd>(registers)
    }
}

/// Enables the virtualization extension, sets up and runs the guest indefinitely.
fn virtualize_core<Arch: Architecture>(registers: &Registers) -> ! {
    log::info!("Initializing the guest");

    // Enable processor's virtualization technology.
    let mut vt = Arch::VirtualizationExtension::default();
    vt.enable();

    // Create a new (empty) guest instance and set up its initial state.
    let id = apic_id::processor_id_from(apic_id::get()).unwrap();
    let guest = &mut Arch::Guest::new(id);
    guest.activate();
    guest.initialize(registers);

    // re-enable interrupts so we don't get deadlocked.
    unsafe { x86::irq::enable() };

    log::info!("Starting the guest");
    loop {
        // Then, run the guest until VM-exit occurs. Some of the events are handled
        // within the architecture specific code and nothing to do here.
        match guest.run() {
            VmExitReason::Cpuid(info) => handle_cpuid(guest, &info),
            VmExitReason::Rdmsr(info) => handle_rdmsr(guest, &info),
            VmExitReason::Wrmsr(info) => handle_wrmsr(guest, &info),
            VmExitReason::XSetBv(info) => handle_xsetbv(guest, &info),
            // Vmcall is made by qemu kvm quests
            VmExitReason::VmCall(info) => handle_vmcall(guest, &info),
            VmExitReason::InitSignal | VmExitReason::StartupIpi | VmExitReason::NestedPageFault => {
            }
        }
    }
}

// passthrough enlightened calls
fn handle_vmcall<T: Guest>(guest: &mut T, info: &VmcallInfo) {
    let guest_rcx = info.rcx;
    let guest_rdx = info.rdx;
    let guest_r8 = info.r8;
    let guest_r9 = info.r9;

    // pass the vmcall to QEMU
    let result_rax: u64;
    unsafe {
        asm!("vmcall", in("rcx") guest_rcx, in("rdx") guest_rdx, in("r8") guest_r8, in("r9") guest_r9, lateout("rax") result_rax);
    }

    guest.regs().rax = result_rax;
    guest.regs().rip = info.instruction_info.next_rip
}

fn handle_custom_vmcall<T: Guest>(guest: &mut T, info: &InstructionInfo) -> bool {
    guest.regs().rip = info.next_rip;
    let call = HypervisorCall::from_bits(guest.regs().rsi as _);
    unsafe { SHARED_HOST_DATA.get_unchecked().vmcall_handler.unwrap()(guest, call) }
}

fn handle_cpuid<T: Guest>(guest: &mut T, info: &InstructionInfo) {
    let leaf = guest.regs().rax as u32;
    let sub_leaf = guest.regs().rcx as u32;

    if sub_leaf == 0x2009 {
        // our CPUID trap
        match handle_custom_vmcall(guest, info) {
            true => {
                guest.regs().rcx = 0x2009; // This leaf indicates that CPUID was handled by the hypervisor.
                return;
            }
            false => {
                // treat as normal
            }
        }
    }

    let mut cpuid_result = cpuid!(leaf, sub_leaf);

    if leaf == 1 {
        // On the Intel processor, CPUID.1.ECX[5] indicates if VT-x is supported.
        // Clear this to prevent other hypervisor tries to use it. On AMD, it is
        // a reserved bit.
        // See: Table 3-10. Feature Information Returned in the ECX Register
        /*cpuid_result.ecx &= !(1 << 5); // hide presence of vmx
        cpuid_result.ecx &= !(1 << 31); // hide presence of hypervisor*/
        // disabled to support hyper-v enlightments
    } else if leaf == HV_CPUID_INTERFACE {
        // Return non "Hv#1" into EAX. This indicate that our hypervisor does NOT
        // conform to the Microsoft hypervisor interface. This prevents the guest
        // from using the interface for optimum performance, but simplifies
        // implementation of our hypervisor. This is required only when testing
        // in the virtualization platform that supports the Microsoft hypervisor
        // interface, such as VMware, and not required for a baremetal.
        // See: Hypervisor Top Level Functional Specification

        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // that would be correct, but not in our case.
        // since we support hyper-v enlightenment, we have to return what QEMU returned to us.
        cpuid_result.eax = SHARED_HOST_DATA.get().unwrap().hv_cpuid_eax;
    }

    guest.regs().rax = u64::from(cpuid_result.eax);
    guest.regs().rbx = u64::from(cpuid_result.ebx);
    guest.regs().rcx = u64::from(cpuid_result.ecx);
    guest.regs().rdx = u64::from(cpuid_result.edx);
    guest.regs().rip = info.next_rip;
}

/// Handles the `RDMSR` instruction for the range not covered by MSR bitmaps.
fn handle_rdmsr<T: Guest>(guest: &mut T, info: &InstructionInfo) {
    let msr = guest.regs().rcx as u32;

    // Passthrough any MSR access. Beware of that VM-exit occurs even for an
    // invalid MSR access which causes #GP(0).
    // See: 26.1.1 Relative Priority of Faults and VM Exits
    //
    // One solution is to catch the exception and inject it into the guest.
    let value = rdmsr(msr);

    guest.regs().rax = value & 0xffff_ffff;
    guest.regs().rdx = value >> 32;
    guest.regs().rip = info.next_rip;
}

/// Handles the `WRMSR` instruction for the range not covered by MSR bitmaps.
fn handle_wrmsr<T: Guest>(guest: &mut T, info: &InstructionInfo) {
    let msr = guest.regs().rcx as u32;
    let value = (guest.regs().rax & 0xffff_ffff) | ((guest.regs().rdx & 0xffff_ffff) << 32);

    // See the comment in `handle_rdmsr`.
    wrmsr(msr, value);

    guest.regs().rip = info.next_rip;
}

// Handles the `XSETBV` instruction.
fn handle_xsetbv<T: Guest>(guest: &mut T, info: &InstructionInfo) {
    let xcr: u32 = guest.regs().rcx as u32;
    let value = (guest.regs().rax & 0xffff_ffff) | ((guest.regs().rdx & 0xffff_ffff) << 32);
    let value = Xcr0::from_bits(value).unwrap();

    // The host CR4 might not have this bit, which is required for executing the
    // `XSETBV` instruction. Set this bit and run the instruction.
    cr4_write(cr4() | Cr4::CR4_ENABLE_OS_XSAVE);

    // XCR may be invalid and this instruction may cause #GP(0). See the comment
    // in `handle_rdmsr`.
    xsetbv(xcr, value);

    guest.regs().rip = info.next_rip;
}

/// Represents a processor architecture that implements hardware-assisted virtualization.
pub(crate) trait Architecture {
    type VirtualizationExtension: Extension;
    type Guest: Guest;
}

/// Represents an implementation of a hardware-assisted virtualization extension.
pub(crate) trait Extension: Default {
    /// Enables the hardware-assisted virtualization extension.
    fn enable(&mut self);
}

/// Represents an implementation of a guest.
pub trait Guest {
    /// Creates an empty uninitialized guest, which must be activated with
    /// `activate` first.
    fn new(id: usize) -> Self
    where
        Self: Sized;

    /// Tells the processor to operate on this guest. Must be called before any
    /// other functions are used.
    fn activate(&mut self);

    /// Initializes the guest based on `registers` and the current system register
    /// values.
    fn initialize(&mut self, registers: &Registers);

    /// Runs the guest until VM-exit occurs.
    fn run(&mut self) -> VmExitReason;

    /// Gets a reference to some of guest registers.
    fn regs(&mut self) -> &mut Registers;

    fn write_response(&mut self, response: HypervisorResponse);
}

/// The reasons of VM-exit and additional information.
pub enum VmExitReason {
    Cpuid(InstructionInfo),
    Rdmsr(InstructionInfo),
    Wrmsr(InstructionInfo),
    XSetBv(InstructionInfo),
    VmCall(VmcallInfo),
    InitSignal,
    StartupIpi,
    NestedPageFault,
}

pub struct VmcallInfo {
    pub rcx: u64,
    pub rdx: u64,
    pub r8: u64,
    pub r9: u64,
    pub instruction_info: InstructionInfo
}

pub struct InstructionInfo {
    /// The next RIP of the guest in case the current instruction is emulated.
    pub next_rip: u64,
}
