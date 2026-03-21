use crate::hypervisor::segments::SegmentDescriptor;
use crate::hypervisor::tables::{InterruptDescriptorTableRaw, InterruptStackTableRaw};
use crate::hypervisor::vmfs::{HvFs, Registers};
use crate::utils::alloc::PoolAlloc;
use crate::utils::intrin::{lar, ldtr, lsl, sgdt, sidt, tr};
use crate::utils::logger::{HvLogger, LogEvent};
use crate::win::MmGetPhysicalAddress;
use crate::{GLOBAL_LOGGER, scoped_log};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use bit_field::BitField;
use core::ptr::addr_of;
use x86::bits64::vmx::{vmclear, vmptrld, vmread, vmwrite, vmxon};
use x86::controlregs::{Cr0, Cr4, cr0, cr0_write, cr3, cr4, cr4_write};
use x86::msr::{IA32_FEATURE_CONTROL, IA32_VMX_BASIC, rdmsr, wrmsr};
use x86::segmentation::{cs, ds, es, fs, gs, ss};
use x86::vmx::{VmFail, vmcs};

#[repr(align(4096), C)]
pub struct Vmcs {
    pub revision: u32,
    pub abort: u32,
}

#[repr(align(4096), C)]
pub struct VmxOn {
    pub revision: u32,
}

impl VmxOn {
    pub fn new() -> Box<VmxOn> {
        let vmx_basic = unsafe { rdmsr(IA32_VMX_BASIC) }.get_bits(0..30);
        let mut me = VmxOn::alloc_sized(4096);

        me.revision = vmx_basic as _;
        me
    }

    pub fn on(&self) -> Result<(), VmFail> {
        let phys = unsafe { MmGetPhysicalAddress(self as *const _ as _) };
        scoped_log!(info, LogEvent::Vmxon(self as *const _ as _, phys));
        unsafe { vmxon(phys) }
    }

    pub fn enable_vmx(&self) -> Result<(), VmFail> {
        let mut msr = unsafe { rdmsr(IA32_FEATURE_CONTROL) };

        // check if VMX is enabled
        if !msr.get_bit(2) {
            // check if msr is locked
            if msr.get_bit(0) {
                return Err(VmFail::VmFailInvalid);
            }

            msr.set_bit(2, true);
            msr.set_bit(0, true);

            unsafe { wrmsr(IA32_FEATURE_CONTROL, msr) };
        }

        unsafe {
            cr0_write(Self::get_adjusted_cr0(cr0()));
            cr4_write(Self::get_adjusted_cr4(cr4()));
        }

        Ok(())
    }

    fn get_adjusted_cr0(cr0: Cr0) -> Cr0 {
        let fixed0 = unsafe { Cr0::from_bits_unchecked(rdmsr(x86::msr::IA32_VMX_CR0_FIXED0) as _) };
        let fixed1 = unsafe { Cr0::from_bits_unchecked(rdmsr(x86::msr::IA32_VMX_CR0_FIXED1) as _) };
        (cr0 & fixed1) | fixed0
    }
    fn get_adjusted_cr4(cr4: Cr4) -> Cr4 {
        let fixed0 = unsafe { Cr4::from_bits_unchecked(rdmsr(x86::msr::IA32_VMX_CR4_FIXED0) as _) };
        let fixed1 = unsafe { Cr4::from_bits_unchecked(rdmsr(x86::msr::IA32_VMX_CR4_FIXED1) as _) };
        (cr4 & fixed1) | fixed0
    }
}

unsafe extern "C" {
    unsafe fn hv_vm_run(regs: &mut Registers);
    unsafe fn hv_vm_exit();
}

impl Vmcs {
    pub fn dump() -> String {
        let me = Self {
            abort: 0,
            revision: 0,
        };

        format!("{:?}", &me)
    }

    pub fn new() -> Box<Vmcs> {
        let vmx_basic = unsafe { rdmsr(IA32_VMX_BASIC) }.get_bits(0..30);
        let mut me = Vmcs::alloc_sized(4096);

        me.revision = vmx_basic as _;
        me.abort = 0;
        me
    }

    pub fn launch(&self, registers: &mut Registers) {
        unsafe { hv_vm_run(registers) }
    }

    pub fn load(&self) -> Result<(), VmFail> {
        let phys = unsafe { MmGetPhysicalAddress(self as *const _ as _) };
        scoped_log!(info, LogEvent::Vmptrld(self as *const _ as _, phys));
        unsafe { vmptrld(phys) }
    }

    pub fn clear(&self) -> Result<(), VmFail> {
        let phys = unsafe { MmGetPhysicalAddress(self as *const _ as _) };
        scoped_log!(info, LogEvent::Vmclear(self as *const _ as _, phys));
        unsafe { vmclear(phys) }
    }

    pub fn vmwrite<T>(field: u32, value: T)
    where
        u64: From<T>,
    {
        unsafe {
            vmwrite(field, u64::from(value)).unwrap() // yes
        }
    }

    pub fn vmread(field: u32) -> u64 {
        unsafe { vmread(field).unwrap() }
    }
}

pub struct HvCpu {
    pub vmcs: Box<Vmcs>,
    pub vmxon: Box<VmxOn>,
    pub hvfs: Box<HvFs>,
    pub stack: Box<[u8; 1024 * 64]>,
    pub idt: Box<InterruptDescriptorTableRaw>,
    pub ist: Box<InterruptStackTableRaw>,
}

impl HvCpu {
    pub fn new(registers: Registers) -> Self {
        Self {
            vmcs: Vmcs::new(),
            vmxon: VmxOn::new(),
            hvfs: Box::new(HvFs::new(registers)),
            // why box::new doesn't place it on heap by default? insane
            stack: unsafe { Box::new_zeroed().assume_init() },
            idt: InterruptDescriptorTableRaw::new(),
            ist: InterruptStackTableRaw::new(),
        }
    }

    pub fn prepare(&self, index: u32) -> Result<(), VmFail> {
        self.vmxon.enable_vmx()?;
        self.vmxon.on()?;

        self.vmcs.clear()?;
        self.vmcs.load()?;

        scoped_log!(
            info,
            LogEvent::ProcessorReady(index, self.hvfs.as_ref() as *const _ as _)
        );

        Ok(())
    }

    pub fn virtualize(&mut self) -> Result<(), VmFail> {
        self.initialize_controls();
        self.initialize_guest();
        self.initialize_host();

        self.vmcs.launch(&mut self.hvfs.registers);

        scoped_log!(info, LogEvent::LaunchingProcessor);

        Ok(())
    }

    fn initialize_host(&self) {
        Vmcs::vmwrite(vmcs::host::ES_SELECTOR, es().bits() & !0b111);
        Vmcs::vmwrite(vmcs::host::CS_SELECTOR, cs().bits() & !0b111);
        Vmcs::vmwrite(vmcs::host::SS_SELECTOR, ss().bits() & !0b111);
        Vmcs::vmwrite(vmcs::host::DS_SELECTOR, ds().bits() & !0b111);
        Vmcs::vmwrite(vmcs::host::FS_SELECTOR, fs().bits() & !0b111);
        Vmcs::vmwrite(vmcs::host::GS_SELECTOR, gs().bits() & !0b111);
        Vmcs::vmwrite(vmcs::host::TR_SELECTOR, tr().bits() & !0b111);

        Vmcs::vmwrite(vmcs::host::CR0, unsafe { cr0() }.bits() as u64);
        Vmcs::vmwrite(vmcs::host::CR3, unsafe { cr3() });
        Vmcs::vmwrite(vmcs::host::CR4, unsafe { cr4() }.bits() as u64);

        Vmcs::vmwrite(vmcs::host::FS_BASE, self.hvfs.as_ref() as *const _ as u64);
        Vmcs::vmwrite(vmcs::host::GS_BASE, unsafe {
            rdmsr(x86::msr::IA32_GS_BASE)
        });

        Vmcs::vmwrite(vmcs::host::TR_BASE, self.ist.as_ref() as *const _ as u64);
        Vmcs::vmwrite(vmcs::host::GDTR_BASE, sgdt().base as u64);
        Vmcs::vmwrite(vmcs::host::IDTR_BASE, self.idt.as_ref() as *const _ as u64);

        Vmcs::vmwrite(vmcs::host::RIP, hv_vm_exit as *const u64 as u64);
        Vmcs::vmwrite(
            vmcs::host::RSP,
            // extra math to point on top of stack
            unsafe { self.stack.as_ptr().byte_offset(1024 * 64) } as u64,
        );
    }

    fn initialize_guest(&self) {
        let idtr = sidt();
        let gdtr = sgdt();

        Vmcs::vmwrite(vmcs::guest::ES_SELECTOR, es().bits());
        Vmcs::vmwrite(vmcs::guest::CS_SELECTOR, cs().bits());
        Vmcs::vmwrite(vmcs::guest::SS_SELECTOR, ss().bits());
        Vmcs::vmwrite(vmcs::guest::DS_SELECTOR, ds().bits());
        Vmcs::vmwrite(vmcs::guest::FS_SELECTOR, fs().bits());
        Vmcs::vmwrite(vmcs::guest::GS_SELECTOR, gs().bits());
        Vmcs::vmwrite(vmcs::guest::TR_SELECTOR, tr().bits());
        Vmcs::vmwrite(vmcs::guest::LDTR_SELECTOR, ldtr().bits());

        Vmcs::vmwrite(vmcs::guest::ES_LIMIT, lsl(es()));
        Vmcs::vmwrite(vmcs::guest::CS_LIMIT, lsl(cs()));
        Vmcs::vmwrite(vmcs::guest::SS_LIMIT, lsl(ss()));
        Vmcs::vmwrite(vmcs::guest::DS_LIMIT, lsl(ds()));
        Vmcs::vmwrite(vmcs::guest::FS_LIMIT, lsl(fs()));
        Vmcs::vmwrite(vmcs::guest::GS_LIMIT, lsl(gs()));
        Vmcs::vmwrite(vmcs::guest::TR_LIMIT, lsl(tr()));

        Vmcs::vmwrite(
            vmcs::guest::ES_ACCESS_RIGHTS,
            Self::access_rights(lar(es())),
        );
        Vmcs::vmwrite(
            vmcs::guest::CS_ACCESS_RIGHTS,
            Self::access_rights(lar(cs())),
        );
        Vmcs::vmwrite(
            vmcs::guest::SS_ACCESS_RIGHTS,
            Self::access_rights(lar(ss())),
        );
        Vmcs::vmwrite(
            vmcs::guest::DS_ACCESS_RIGHTS,
            Self::access_rights(lar(ds())),
        );
        Vmcs::vmwrite(
            vmcs::guest::FS_ACCESS_RIGHTS,
            Self::access_rights(lar(fs())),
        );
        Vmcs::vmwrite(
            vmcs::guest::GS_ACCESS_RIGHTS,
            Self::access_rights(lar(gs())),
        );
        Vmcs::vmwrite(
            vmcs::guest::TR_ACCESS_RIGHTS,
            Self::access_rights(lar(tr())),
        );
        Vmcs::vmwrite(vmcs::guest::LDTR_ACCESS_RIGHTS, Self::access_rights(0));

        Vmcs::vmwrite(vmcs::guest::FS_BASE, unsafe {
            rdmsr(x86::msr::IA32_FS_BASE)
        });
        Vmcs::vmwrite(vmcs::guest::GS_BASE, unsafe {
            rdmsr(x86::msr::IA32_GS_BASE)
        });

        Vmcs::vmwrite(
            vmcs::guest::TR_BASE,
            SegmentDescriptor::try_from_gdtr(&gdtr, tr())
                .unwrap()
                .base(),
        );

        Vmcs::vmwrite(vmcs::guest::GDTR_BASE, gdtr.base as u64);
        Vmcs::vmwrite(vmcs::guest::GDTR_LIMIT, gdtr.limit);
        Vmcs::vmwrite(vmcs::guest::IDTR_BASE, idtr.base as u64);
        Vmcs::vmwrite(vmcs::guest::IDTR_LIMIT, idtr.limit);

        // poor intel and their sysenter instruction nobody cared about
        Vmcs::vmwrite(vmcs::guest::IA32_SYSENTER_CS, unsafe {
            rdmsr(x86::msr::IA32_SYSENTER_CS)
        });
        Vmcs::vmwrite(vmcs::guest::IA32_SYSENTER_EIP, unsafe {
            rdmsr(x86::msr::IA32_SYSENTER_EIP)
        });
        Vmcs::vmwrite(vmcs::guest::IA32_SYSENTER_ESP, unsafe {
            rdmsr(x86::msr::IA32_SYSENTER_ESP)
        });

        Vmcs::vmwrite(vmcs::guest::LINK_PTR_FULL, u64::MAX);

        Vmcs::vmwrite(vmcs::guest::CR0, unsafe { cr0() }.bits() as u64);
        Vmcs::vmwrite(vmcs::guest::CR3, unsafe { cr3() });
        Vmcs::vmwrite(vmcs::guest::CR4, unsafe { cr4() }.bits() as u64);
        Vmcs::vmwrite(vmcs::guest::RSP, self.hvfs.registers.rsp);
        Vmcs::vmwrite(vmcs::guest::RIP, self.hvfs.registers.rip);
        Vmcs::vmwrite(vmcs::guest::RFLAGS, self.hvfs.registers.rflags);
    }

    fn initialize_controls(&self) {
        Vmcs::vmwrite(
            vmcs::control::VMEXIT_CONTROLS,
            Self::adjust_vmx_control(
                VmxControl::VmExit,
                (vmcs::control::ExitControls::HOST_ADDRESS_SPACE_SIZE.bits()) as u64,
            ),
        );

        Vmcs::vmwrite(
            vmcs::control::VMENTRY_CONTROLS,
            Self::adjust_vmx_control(
                VmxControl::VmEntry,
                vmcs::control::EntryControls::IA32E_MODE_GUEST.bits() as u64,
            ),
        );

        // nothing to enable in the PINBASED_EXEC_CONTROLS.
        Vmcs::vmwrite(
            vmcs::control::PINBASED_EXEC_CONTROLS,
            Self::adjust_vmx_control(VmxControl::PinBased, 0),
        );

        Vmcs::vmwrite(
            vmcs::control::PRIMARY_PROCBASED_EXEC_CONTROLS,
            Self::adjust_vmx_control(
                VmxControl::ProcessorBased,
                (vmcs::control::PrimaryControls::SECONDARY_CONTROLS.bits()) as u64,
            ),
        );
        // ENABLE_USER_WAIT_PAUSE is required for win11+
        Vmcs::vmwrite(
            vmcs::control::SECONDARY_PROCBASED_EXEC_CONTROLS,
            Self::adjust_vmx_control(
                VmxControl::ProcessorBased2,
                (vmcs::control::SecondaryControls::ENABLE_RDTSCP
                    | vmcs::control::SecondaryControls::ENABLE_INVPCID
                    | vmcs::control::SecondaryControls::ENABLE_XSAVES_XRSTORS
                    | vmcs::control::SecondaryControls::ENABLE_USER_WAIT_PAUSE)
                    .bits() as u64,
            ),
        );
    }

    // weird vmx quirks
    fn access_rights(access_rights: u32) -> u32 {
        const VMX_SEGMENT_ACCESS_RIGHTS_UNUSABLE_FLAG: u32 = 1 << 16;
        if access_rights == 0 {
            return VMX_SEGMENT_ACCESS_RIGHTS_UNUSABLE_FLAG;
        }

        (access_rights >> 8) & 0b1111_0000_1111_1111
    }

    fn adjust_vmx_control(control: VmxControl, requested_value: u64) -> u64 {
        const IA32_VMX_BASIC_VMX_CONTROLS_FLAG: u64 = 1 << 55;

        let vmx_basic = unsafe { rdmsr(IA32_VMX_BASIC) };
        let true_cap_msr_supported = (vmx_basic & IA32_VMX_BASIC_VMX_CONTROLS_FLAG) != 0;

        let cap_msr = match (control, true_cap_msr_supported) {
            (VmxControl::PinBased, true) => x86::msr::IA32_VMX_TRUE_PINBASED_CTLS,
            (VmxControl::PinBased, false) => x86::msr::IA32_VMX_PINBASED_CTLS,
            (VmxControl::ProcessorBased, true) => x86::msr::IA32_VMX_TRUE_PROCBASED_CTLS,
            (VmxControl::ProcessorBased, false) => x86::msr::IA32_VMX_PROCBASED_CTLS,
            (VmxControl::VmExit, true) => x86::msr::IA32_VMX_TRUE_EXIT_CTLS,
            (VmxControl::VmExit, false) => x86::msr::IA32_VMX_EXIT_CTLS,
            (VmxControl::VmEntry, true) => x86::msr::IA32_VMX_TRUE_ENTRY_CTLS,
            (VmxControl::VmEntry, false) => x86::msr::IA32_VMX_ENTRY_CTLS,
            (VmxControl::ProcessorBased2, _) => x86::msr::IA32_VMX_PROCBASED_CTLS2,
            (VmxControl::ProcessorBased3, _) => {
                const IA32_VMX_PROCBASED_CTLS3: u32 = 0x492;

                let allowed1 = unsafe { rdmsr(IA32_VMX_PROCBASED_CTLS3) };
                let effective_value = requested_value & allowed1;
                // assert!(
                //     effective_value | requested_value == effective_value,
                //     "One or more requested features are not supported: {effective_value:#x?} : {requested_value:#x?} "
                // );
                return effective_value;
            }
        };

        let capabilities = unsafe { rdmsr(cap_msr) };
        let allowed0 = capabilities as u32;
        let allowed1 = (capabilities >> 32) as u32;
        let requested_value = u32::try_from(requested_value).unwrap();
        let mut effective_value = requested_value;
        effective_value |= allowed0;
        effective_value &= allowed1;
        // assert!(
        //     effective_value | requested_value == effective_value,
        //     "One or more requested features are not supported for {control:?}: {effective_value:#x?} vs {requested_value:#x?}"
        // );
        u64::from(effective_value)
    }
}

#[expect(dead_code)]
#[derive(Clone, Copy, Debug)]
enum VmxControl {
    PinBased,
    ProcessorBased,
    ProcessorBased2,
    ProcessorBased3,
    VmExit,
    VmEntry,
}

const VMCS_CONTROL_HLAT_PREFIX_SIZE: u32 = 0x6;
const VMCS_CONTROL_LAST_PID_POINTER_INDEX: u32 = 0x8;
const VMCS_GUEST_UINV: u32 = 0x814;
const VMCS_CONTROL_TERTIARY_PROCESSOR_BASED_VM_EXECUTION_CONTROLS: u32 = 0x2034;
const VMCS_CONTROL_ENCLV_EXITING_BITMAP: u32 = 0x2036;
const VMCS_CONTROL_LOW_PASID_DIRECTORY_ADDRESS: u32 = 0x2038;
const VMCS_CONTROL_HIGH_PASID_DIRECTORY_ADDRESS: u32 = 0x203A;
const VMCS_CONTROL_SHARED_EPT_POINTER: u32 = 0x203C;
const VMCS_CONTROL_PCONFIG_EXITING_BITMAP: u32 = 0x203E;
const VMCS_CONTROL_HLATP: u32 = 0x2040;
const VMCS_CONTROL_PID_POINTER_TABLE_ADDRESS: u32 = 0x2042;
const VMCS_CONTROL_SECONDARY_VM_EXIT_CONTROLS: u32 = 0x2044;
const VMCS_CONTROL_IA32_SPEC_CTRL_MASK: u32 = 0x204A;
const VMCS_CONTROL_IA32_SPEC_CTRL_SHADOW: u32 = 0x204C;
const VMCS_GUEST_IA32_LBR_CTL: u32 = 0x2816;
const VMCS_GUEST_IA32_PKRS: u32 = 0x2818;
const VMCS_HOST_IA32_PKRS: u32 = 0x2C06;
const VMCS_CONTROL_INSTRUCTION_TIMEOUT_CONTROL: u32 = 0x4024;
const VMCS_GUEST_IA32_S_CET: u32 = 0x6828;
const VMCS_GUEST_SSP: u32 = 0x682A;
const VMCS_GUEST_IA32_INTERRUPT_SSP_TABLE_ADDR: u32 = 0x682C;
const VMCS_HOST_IA32_S_CET: u32 = 0x6C18;
const VMCS_HOST_SSP: u32 = 0x6C1A;
const VMCS_HOST_IA32_INTERRUPT_SSP_TABLE_ADDR: u32 = 0x6C1C;

// taken from Barevisor
impl core::fmt::Debug for Vmcs {
    #[rustfmt::skip]
    #[expect(clippy::too_many_lines)]
    fn fmt(&self, format: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // different from Vmcs::vmread, this one doesnt error out.
        fn vmread_relaxed(encoding: u32) -> u64 {
            unsafe { vmread(encoding) }.unwrap_or(0)
        }

        format.debug_struct("Vmcs")
        .field("Current VMCS                                   ", &addr_of!(self.revision))
        .field("Revision ID                                    ", &self.revision)

        // 16-Bit Guest-State Fields
        .field("Guest ES Selector                              ", &vmread_relaxed(vmcs::guest::ES_SELECTOR))
        .field("Guest CS Selector                              ", &vmread_relaxed(vmcs::guest::CS_SELECTOR))
        .field("Guest SS Selector                              ", &vmread_relaxed(vmcs::guest::SS_SELECTOR))
        .field("Guest DS Selector                              ", &vmread_relaxed(vmcs::guest::DS_SELECTOR))
        .field("Guest FS Selector                              ", &vmread_relaxed(vmcs::guest::FS_SELECTOR))
        .field("Guest GS Selector                              ", &vmread_relaxed(vmcs::guest::GS_SELECTOR))
        .field("Guest LDTR Selector                            ", &vmread_relaxed(vmcs::guest::LDTR_SELECTOR))
        .field("Guest TR Selector                              ", &vmread_relaxed(vmcs::guest::TR_SELECTOR))
        .field("Guest interrupt status                         ", &vmread_relaxed(vmcs::guest::INTERRUPT_STATUS))
        .field("PML index                                      ", &vmread_relaxed(vmcs::guest::PML_INDEX))
        .field("Guest UINV                                     ", &vmread_relaxed(VMCS_GUEST_UINV))

        // 64-Bit Guest-State Fields
        .field("VMCS link pointer                              ", &vmread_relaxed(vmcs::guest::LINK_PTR_FULL))
        .field("Guest IA32_DEBUGCTL                            ", &vmread_relaxed(vmcs::guest::IA32_DEBUGCTL_FULL))
        .field("Guest IA32_PAT                                 ", &vmread_relaxed(vmcs::guest::IA32_PAT_FULL))
        .field("Guest IA32_EFER                                ", &vmread_relaxed(vmcs::guest::IA32_EFER_FULL))
        .field("Guest IA32_PERF_GLOBAL_CTRL                    ", &vmread_relaxed(vmcs::guest::IA32_PERF_GLOBAL_CTRL_FULL))
        .field("Guest PDPTE0                                   ", &vmread_relaxed(vmcs::guest::PDPTE0_FULL))
        .field("Guest PDPTE1                                   ", &vmread_relaxed(vmcs::guest::PDPTE1_FULL))
        .field("Guest PDPTE2                                   ", &vmread_relaxed(vmcs::guest::PDPTE2_FULL))
        .field("Guest PDPTE3                                   ", &vmread_relaxed(vmcs::guest::PDPTE3_FULL))
        .field("Guest IA32_BNDCFGS                             ", &vmread_relaxed(vmcs::guest::IA32_BNDCFGS_FULL))
        .field("Guest IA32_RTIT_CTL                            ", &vmread_relaxed(vmcs::guest::IA32_RTIT_CTL_FULL))
        .field("Guest IA32_LBR_CTL                             ", &vmread_relaxed(VMCS_GUEST_IA32_LBR_CTL))
        .field("Guest IA32_PKRS                                ", &vmread_relaxed(VMCS_GUEST_IA32_PKRS))

        // 32-Bit Guest-State Fields
        .field("Guest ES Limit                                 ", &vmread_relaxed(vmcs::guest::ES_LIMIT))
        .field("Guest CS Limit                                 ", &vmread_relaxed(vmcs::guest::CS_LIMIT))
        .field("Guest SS Limit                                 ", &vmread_relaxed(vmcs::guest::SS_LIMIT))
        .field("Guest DS Limit                                 ", &vmread_relaxed(vmcs::guest::DS_LIMIT))
        .field("Guest FS Limit                                 ", &vmread_relaxed(vmcs::guest::FS_LIMIT))
        .field("Guest GS Limit                                 ", &vmread_relaxed(vmcs::guest::GS_LIMIT))
        .field("Guest LDTR Limit                               ", &vmread_relaxed(vmcs::guest::LDTR_LIMIT))
        .field("Guest TR Limit                                 ", &vmread_relaxed(vmcs::guest::TR_LIMIT))
        .field("Guest GDTR limit                               ", &vmread_relaxed(vmcs::guest::GDTR_LIMIT))
        .field("Guest IDTR limit                               ", &vmread_relaxed(vmcs::guest::IDTR_LIMIT))
        .field("Guest ES access rights                         ", &vmread_relaxed(vmcs::guest::ES_ACCESS_RIGHTS))
        .field("Guest CS access rights                         ", &vmread_relaxed(vmcs::guest::CS_ACCESS_RIGHTS))
        .field("Guest SS access rights                         ", &vmread_relaxed(vmcs::guest::SS_ACCESS_RIGHTS))
        .field("Guest DS access rights                         ", &vmread_relaxed(vmcs::guest::DS_ACCESS_RIGHTS))
        .field("Guest FS access rights                         ", &vmread_relaxed(vmcs::guest::FS_ACCESS_RIGHTS))
        .field("Guest GS access rights                         ", &vmread_relaxed(vmcs::guest::GS_ACCESS_RIGHTS))
        .field("Guest LDTR access rights                       ", &vmread_relaxed(vmcs::guest::LDTR_ACCESS_RIGHTS))
        .field("Guest TR access rights                         ", &vmread_relaxed(vmcs::guest::TR_ACCESS_RIGHTS))
        .field("Guest interruptibility state                   ", &vmread_relaxed(vmcs::guest::INTERRUPTIBILITY_STATE))
        .field("Guest activity state                           ", &vmread_relaxed(vmcs::guest::ACTIVITY_STATE))
        .field("Guest SMBASE                                   ", &vmread_relaxed(vmcs::guest::SMBASE))
        .field("Guest IA32_SYSENTER_CS                         ", &vmread_relaxed(vmcs::guest::IA32_SYSENTER_CS))
        .field("VMX-preemption timer value                     ", &vmread_relaxed(vmcs::guest::VMX_PREEMPTION_TIMER_VALUE))

        // Natural-Width Guest-State Fields
        .field("Guest CR0                                      ", &vmread_relaxed(vmcs::guest::CR0))
        .field("Guest CR3                                      ", &vmread_relaxed(vmcs::guest::CR3))
        .field("Guest CR4                                      ", &vmread_relaxed(vmcs::guest::CR4))
        .field("Guest ES Base                                  ", &vmread_relaxed(vmcs::guest::ES_BASE))
        .field("Guest CS Base                                  ", &vmread_relaxed(vmcs::guest::CS_BASE))
        .field("Guest SS Base                                  ", &vmread_relaxed(vmcs::guest::SS_BASE))
        .field("Guest DS Base                                  ", &vmread_relaxed(vmcs::guest::DS_BASE))
        .field("Guest FS Base                                  ", &vmread_relaxed(vmcs::guest::FS_BASE))
        .field("Guest GS Base                                  ", &vmread_relaxed(vmcs::guest::GS_BASE))
        .field("Guest LDTR base                                ", &vmread_relaxed(vmcs::guest::LDTR_BASE))
        .field("Guest TR base                                  ", &vmread_relaxed(vmcs::guest::TR_BASE))
        .field("Guest GDTR base                                ", &vmread_relaxed(vmcs::guest::GDTR_BASE))
        .field("Guest IDTR base                                ", &vmread_relaxed(vmcs::guest::IDTR_BASE))
        .field("Guest DR7                                      ", &vmread_relaxed(vmcs::guest::DR7))
        .field("Guest RSP                                      ", &vmread_relaxed(vmcs::guest::RSP))
        .field("Guest RIP                                      ", &vmread_relaxed(vmcs::guest::RIP))
        .field("Guest RFLAGS                                   ", &vmread_relaxed(vmcs::guest::RFLAGS))
        .field("Guest pending debug exceptions                 ", &vmread_relaxed(vmcs::guest::PENDING_DBG_EXCEPTIONS))
        .field("Guest IA32_SYSENTER_ESP                        ", &vmread_relaxed(vmcs::guest::IA32_SYSENTER_ESP))
        .field("Guest IA32_SYSENTER_EIP                        ", &vmread_relaxed(vmcs::guest::IA32_SYSENTER_EIP))
        .field("Guest IA32_S_CET                               ", &vmread_relaxed(VMCS_GUEST_IA32_S_CET))
        .field("Guest SSP                                      ", &vmread_relaxed(VMCS_GUEST_SSP))
        .field("Guest IA32_INTERRUPT_SSP_TABLE_ADDR            ", &vmread_relaxed(VMCS_GUEST_IA32_INTERRUPT_SSP_TABLE_ADDR))

        // 16-Bit Host-State Fields
        .field("Host ES Selector                               ", &vmread_relaxed(vmcs::host::ES_SELECTOR))
        .field("Host CS Selector                               ", &vmread_relaxed(vmcs::host::CS_SELECTOR))
        .field("Host SS Selector                               ", &vmread_relaxed(vmcs::host::SS_SELECTOR))
        .field("Host DS Selector                               ", &vmread_relaxed(vmcs::host::DS_SELECTOR))
        .field("Host FS Selector                               ", &vmread_relaxed(vmcs::host::FS_SELECTOR))
        .field("Host GS Selector                               ", &vmread_relaxed(vmcs::host::GS_SELECTOR))
        .field("Host TR Selector                               ", &vmread_relaxed(vmcs::host::TR_SELECTOR))

        // 64-Bit Host-State Fields
        .field("Host IA32_PAT                                  ", &vmread_relaxed(vmcs::host::IA32_PAT_FULL))
        .field("Host IA32_EFER                                 ", &vmread_relaxed(vmcs::host::IA32_EFER_FULL))
        .field("Host IA32_PERF_GLOBAL_CTRL                     ", &vmread_relaxed(vmcs::host::IA32_PERF_GLOBAL_CTRL_FULL))
        .field("Host IA32_PKRS                                 ", &vmread_relaxed(VMCS_HOST_IA32_PKRS))

        // 32-Bit Host-State Fields
        .field("Host IA32_SYSENTER_CS                          ", &vmread_relaxed(vmcs::host::IA32_SYSENTER_CS))

        // Natural-Width Host-State Fields
        .field("Host CR0                                       ", &vmread_relaxed(vmcs::host::CR0))
        .field("Host CR3                                       ", &vmread_relaxed(vmcs::host::CR3))
        .field("Host CR4                                       ", &vmread_relaxed(vmcs::host::CR4))
        .field("Host FS Base                                   ", &vmread_relaxed(vmcs::host::FS_BASE))
        .field("Host GS Base                                   ", &vmread_relaxed(vmcs::host::GS_BASE))
        .field("Host TR base                                   ", &vmread_relaxed(vmcs::host::TR_BASE))
        .field("Host GDTR base                                 ", &vmread_relaxed(vmcs::host::GDTR_BASE))
        .field("Host IDTR base                                 ", &vmread_relaxed(vmcs::host::IDTR_BASE))
        .field("Host IA32_SYSENTER_ESP                         ", &vmread_relaxed(vmcs::host::IA32_SYSENTER_ESP))
        .field("Host IA32_SYSENTER_EIP                         ", &vmread_relaxed(vmcs::host::IA32_SYSENTER_EIP))
        .field("Host RSP                                       ", &vmread_relaxed(vmcs::host::RSP))
        .field("Host RIP                                       ", &vmread_relaxed(vmcs::host::RIP))
        .field("Host IA32_S_CET                                ", &vmread_relaxed(VMCS_HOST_IA32_S_CET))
        .field("Host SSP                                       ", &vmread_relaxed(VMCS_HOST_SSP))
        .field("Host IA32_INTERRUPT_SSP_TABLE_ADDR             ", &vmread_relaxed(VMCS_HOST_IA32_INTERRUPT_SSP_TABLE_ADDR))

        // 16-Bit Control Fields
        .field("Virtual-processor identifier                   ", &vmread_relaxed(vmcs::control::VPID))
        .field("Posted-interrupt notification vector           ", &vmread_relaxed(vmcs::control::POSTED_INTERRUPT_NOTIFICATION_VECTOR))
        .field("EPTP index                                     ", &vmread_relaxed(vmcs::control::EPTP_INDEX))
        .field("HLAT prefix size                               ", &vmread_relaxed(VMCS_CONTROL_HLAT_PREFIX_SIZE))
        .field("Last PID-pointer index                         ", &vmread_relaxed(VMCS_CONTROL_LAST_PID_POINTER_INDEX))

        // 64-Bit Control Fields
        .field("Address of I/O bitmap A                        ", &vmread_relaxed(vmcs::control::IO_BITMAP_A_ADDR_FULL))
        .field("Address of I/O bitmap B                        ", &vmread_relaxed(vmcs::control::IO_BITMAP_B_ADDR_FULL))
        .field("Address of MSR bitmaps                         ", &vmread_relaxed(vmcs::control::MSR_BITMAPS_ADDR_FULL))
        .field("VM-exit MSR-store address                      ", &vmread_relaxed(vmcs::control::VMEXIT_MSR_STORE_ADDR_FULL))
        .field("VM-exit MSR-load address                       ", &vmread_relaxed(vmcs::control::VMEXIT_MSR_LOAD_ADDR_FULL))
        .field("VM-entry MSR-load address                      ", &vmread_relaxed(vmcs::control::VMENTRY_MSR_LOAD_ADDR_FULL))
        .field("Executive-VMCS pointer                         ", &vmread_relaxed(vmcs::control::EXECUTIVE_VMCS_PTR_FULL))
        .field("PML address                                    ", &vmread_relaxed(vmcs::control::PML_ADDR_FULL))
        .field("TSC offset                                     ", &vmread_relaxed(vmcs::control::TSC_OFFSET_FULL))
        .field("Virtual-APIC address                           ", &vmread_relaxed(vmcs::control::VIRT_APIC_ADDR_FULL))
        .field("APIC-access address                            ", &vmread_relaxed(vmcs::control::APIC_ACCESS_ADDR_FULL))
        .field("Posted-interrupt descriptor address            ", &vmread_relaxed(vmcs::control::POSTED_INTERRUPT_DESC_ADDR_FULL))
        .field("VM-function controls                           ", &vmread_relaxed(vmcs::control::VM_FUNCTION_CONTROLS_FULL))
        .field("EPT pointer                                    ", &vmread_relaxed(vmcs::control::EPTP_FULL))
        .field("EOI-exit bitmap 0                              ", &vmread_relaxed(vmcs::control::EOI_EXIT0_FULL))
        .field("EOI-exit bitmap 1                              ", &vmread_relaxed(vmcs::control::EOI_EXIT1_FULL))
        .field("EOI-exit bitmap 2                              ", &vmread_relaxed(vmcs::control::EOI_EXIT2_FULL))
        .field("EOI-exit bitmap 3                              ", &vmread_relaxed(vmcs::control::EOI_EXIT3_FULL))
        .field("EPTP-list address                              ", &vmread_relaxed(vmcs::control::EPTP_LIST_ADDR_FULL))
        .field("VMREAD-bitmap address                          ", &vmread_relaxed(vmcs::control::VMREAD_BITMAP_ADDR_FULL))
        .field("VMWRITE-bitmap address                         ", &vmread_relaxed(vmcs::control::VMWRITE_BITMAP_ADDR_FULL))
        .field("Virtualization-exception information address   ", &vmread_relaxed(vmcs::control::VIRT_EXCEPTION_INFO_ADDR_FULL))
        .field("XSS-exiting bitmap                             ", &vmread_relaxed(vmcs::control::XSS_EXITING_BITMAP_FULL))
        .field("ENCLS-exiting bitmap                           ", &vmread_relaxed(vmcs::control::ENCLS_EXITING_BITMAP_FULL))
        .field("Sub-page-permission-table pointer              ", &vmread_relaxed(vmcs::control::SUBPAGE_PERM_TABLE_PTR_FULL))
        .field("TSC multiplier                                 ", &vmread_relaxed(vmcs::control::TSC_MULTIPLIER_FULL))
        .field("Tertiary processor-based VM-execution controls ", &vmread_relaxed(VMCS_CONTROL_TERTIARY_PROCESSOR_BASED_VM_EXECUTION_CONTROLS))
        .field("ENCLV-exiting bitmap                           ", &vmread_relaxed(VMCS_CONTROL_ENCLV_EXITING_BITMAP))
        .field("Low PASID directory address                    ", &vmread_relaxed(VMCS_CONTROL_LOW_PASID_DIRECTORY_ADDRESS))
        .field("High PASID directory address                   ", &vmread_relaxed(VMCS_CONTROL_HIGH_PASID_DIRECTORY_ADDRESS))
        .field("Shared EPT pointer                             ", &vmread_relaxed(VMCS_CONTROL_SHARED_EPT_POINTER))
        .field("PCONFIG-exiting bitmap                         ", &vmread_relaxed(VMCS_CONTROL_PCONFIG_EXITING_BITMAP))
        .field("HLATP                                          ", &vmread_relaxed(VMCS_CONTROL_HLATP))
        .field("PID-pointer table address                      ", &vmread_relaxed(VMCS_CONTROL_PID_POINTER_TABLE_ADDRESS))
        .field("Secondary VM-exit controls                     ", &vmread_relaxed(VMCS_CONTROL_SECONDARY_VM_EXIT_CONTROLS))
        .field("IA32_SPEC_CTRL mask                            ", &vmread_relaxed(VMCS_CONTROL_IA32_SPEC_CTRL_MASK))
        .field("IA32_SPEC_CTRL shadow                          ", &vmread_relaxed(VMCS_CONTROL_IA32_SPEC_CTRL_SHADOW))

        // 32-Bit Control Fields
        .field("Pin-based VM-execution controls                ", &vmread_relaxed(vmcs::control::PINBASED_EXEC_CONTROLS))
        .field("Primary processor-based VM-execution controls  ", &vmread_relaxed(vmcs::control::PRIMARY_PROCBASED_EXEC_CONTROLS))
        .field("Exception bitmap                               ", &vmread_relaxed(vmcs::control::EXCEPTION_BITMAP))
        .field("Page-fault error-code mask                     ", &vmread_relaxed(vmcs::control::PAGE_FAULT_ERR_CODE_MASK))
        .field("Page-fault error-code match                    ", &vmread_relaxed(vmcs::control::PAGE_FAULT_ERR_CODE_MATCH))
        .field("CR3-target count                               ", &vmread_relaxed(vmcs::control::CR3_TARGET_COUNT))
        .field("Primary VM-exit controls                       ", &vmread_relaxed(vmcs::control::VMEXIT_CONTROLS))
        .field("VM-exit MSR-store count                        ", &vmread_relaxed(vmcs::control::VMEXIT_MSR_STORE_COUNT))
        .field("VM-exit MSR-load count                         ", &vmread_relaxed(vmcs::control::VMEXIT_MSR_LOAD_COUNT))
        .field("VM-entry controls                              ", &vmread_relaxed(vmcs::control::VMENTRY_CONTROLS))
        .field("VM-entry MSR-load count                        ", &vmread_relaxed(vmcs::control::VMENTRY_MSR_LOAD_COUNT))
        .field("VM-entry interruption-information field        ", &vmread_relaxed(vmcs::control::VMENTRY_INTERRUPTION_INFO_FIELD))
        .field("VM-entry exception error code                  ", &vmread_relaxed(vmcs::control::VMENTRY_EXCEPTION_ERR_CODE))
        .field("VM-entry instruction length                    ", &vmread_relaxed(vmcs::control::VMENTRY_INSTRUCTION_LEN))
        .field("TPR threshold                                  ", &vmread_relaxed(vmcs::control::TPR_THRESHOLD))
        .field("Secondary processor-based VM-execution controls", &vmread_relaxed(vmcs::control::SECONDARY_PROCBASED_EXEC_CONTROLS))
        .field("PLE_Gap                                        ", &vmread_relaxed(vmcs::control::PLE_GAP))
        .field("PLE_Window                                     ", &vmread_relaxed(vmcs::control::PLE_WINDOW))
        .field("Instruction-timeout control                    ", &vmread_relaxed(VMCS_CONTROL_INSTRUCTION_TIMEOUT_CONTROL))

        // Natural-Width Control Fields
        .field("CR0 guest/host mask                            ", &vmread_relaxed(vmcs::control::CR0_GUEST_HOST_MASK))
        .field("CR4 guest/host mask                            ", &vmread_relaxed(vmcs::control::CR4_GUEST_HOST_MASK))
        .field("CR0 read shadow                                ", &vmread_relaxed(vmcs::control::CR0_READ_SHADOW))
        .field("CR4 read shadow                                ", &vmread_relaxed(vmcs::control::CR4_READ_SHADOW))
        .field("CR3-target value 0                             ", &vmread_relaxed(vmcs::control::CR3_TARGET_VALUE0))
        .field("CR3-target value 1                             ", &vmread_relaxed(vmcs::control::CR3_TARGET_VALUE1))
        .field("CR3-target value 2                             ", &vmread_relaxed(vmcs::control::CR3_TARGET_VALUE2))
        .field("CR3-target value 3                             ", &vmread_relaxed(vmcs::control::CR3_TARGET_VALUE3))

        // 16-Bit Read-Only Data Fields

        // 64-Bit Read-Only Data Fields
        .field("Guest-physical address                         ", &vmread_relaxed(vmcs::ro::GUEST_PHYSICAL_ADDR_FULL))

        // 32-Bit Read-Only Data Fields
        .field("VM-instruction error                           ", &vmread_relaxed(vmcs::ro::VM_INSTRUCTION_ERROR))
        .field("Exit reason                                    ", &vmread_relaxed(vmcs::ro::EXIT_REASON))
        .field("VM-exit interruption information               ", &vmread_relaxed(vmcs::ro::VMEXIT_INTERRUPTION_INFO))
        .field("VM-exit interruption error code                ", &vmread_relaxed(vmcs::ro::VMEXIT_INTERRUPTION_ERR_CODE))
        .field("IDT-vectoring information field                ", &vmread_relaxed(vmcs::ro::IDT_VECTORING_INFO))
        .field("IDT-vectoring error code                       ", &vmread_relaxed(vmcs::ro::IDT_VECTORING_ERR_CODE))
        .field("VM-exit instruction length                     ", &vmread_relaxed(vmcs::ro::VMEXIT_INSTRUCTION_LEN))
        .field("VM-exit instruction information                ", &vmread_relaxed(vmcs::ro::VMEXIT_INSTRUCTION_INFO))

        // Natural-Width Read-Only Data Fields
        .field("Exit qualification                             ", &vmread_relaxed(vmcs::ro::EXIT_QUALIFICATION))
        .field("I/O RCX                                        ", &vmread_relaxed(vmcs::ro::IO_RCX))
        .field("I/O RSI                                        ", &vmread_relaxed(vmcs::ro::IO_RSI))
        .field("I/O RDI                                        ", &vmread_relaxed(vmcs::ro::IO_RDI))
        .field("I/O RIP                                        ", &vmread_relaxed(vmcs::ro::IO_RIP))
        .field("Guest-linear address                           ", &vmread_relaxed(vmcs::ro::GUEST_LINEAR_ADDR))
        .finish_non_exhaustive()
    }
}
