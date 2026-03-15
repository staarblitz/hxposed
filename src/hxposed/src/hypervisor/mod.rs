use crate::hypervisor::ops::PlatformOps;
use crate::hypervisor::vcpu::HvCpu;
use crate::hypervisor::vmfs::{capture_context, Registers};
use alloc::vec::Vec;
use core::arch::{asm, global_asm};
use bit_field::BitField;
use core::sync::atomic::{AtomicU64, Ordering};
use x86::vmx::VmFail;
use crate::utils::intrin;

pub(crate) mod init;
mod ops;
mod vcpu;
mod vmexit;
pub mod vmfs;
mod segments;

// Most of credit goes to Satoshi Tanda's Barevisor project
// I just supported Windows 11, beautified it, and "rewrote" it for HxPosed.,

pub struct Hypervisor {
    pub cpus: Vec<HvCpu>,
}

static VIRTUALIZE_BITMAP: AtomicU64 = AtomicU64::new(0);

global_asm!(include_str!("regs.inc"));
global_asm!(include_str!("context.asm"));
global_asm!(include_str!("vmexit.asm"));
impl Hypervisor {
    #[unsafe(no_mangle)]
    pub fn virtualize_system(&mut self) {
        PlatformOps::run_on_all_processors(|index| {
            log::info!("Virtualizing processor {index}");

            let mut registers = Registers::default();
            unsafe {capture_context(&mut registers)};

            let bmp = VIRTUALIZE_BITMAP.load(Ordering::Acquire);
            if !bmp.get_bit(index as _) {
                log::info!("Processor {index} is not virtualized");
                let mut cpu = HvCpu::new(registers);

                VIRTUALIZE_BITMAP.store(*bmp.clone().set_bit(index as _, true), Ordering::Release);

                // unwrap rocks
                cpu.prepare().unwrap();
                cpu.virtualize().unwrap();

                //self.cpus.push(cpu);
            }

            log::info!("Processor {index} is virtualized");
        });
    }
}
