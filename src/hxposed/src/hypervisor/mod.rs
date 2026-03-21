use crate::hypervisor::ops::PlatformOps;
use crate::hypervisor::vcpu::HvCpu;
use crate::hypervisor::vmfs::{Registers, capture_context};
use crate::utils::intrin;
use crate::utils::logger::LogEvent;
use alloc::vec::Vec;
use bit_field::BitField;
use core::arch::{asm, global_asm};
use core::sync::atomic::{AtomicU64, Ordering};
use x86::vmx::VmFail;

pub(crate) mod init;
pub(crate) mod ops;
pub(crate) mod segments;
pub(crate) mod vcpu;
pub(crate) mod vmexit;
pub mod vmfs;
mod tables;
// Most of credit goes to Satoshi Tanda's Barevisor project
// I just supported Windows 11, beautified it, and "rewrote" it for HxPosed.,

pub struct Hypervisor {
    pub cpus: Vec<HvCpu>,
}

static VIRTUALIZE_BITMAP: AtomicU64 = AtomicU64::new(0);

global_asm!(include_str!("regs.inc"));
global_asm!(include_str!("context.asm"));
global_asm!(include_str!("vmexit.asm"));
global_asm!(include_str!("idt.asm"));
impl Hypervisor {
    #[unsafe(no_mangle)]
    pub fn virtualize_system(&mut self) {
        PlatformOps::run_on_all_processors(|index| {
            let mut registers = Registers::default();
            unsafe { capture_context(&mut registers) };

            let bmp = VIRTUALIZE_BITMAP.load(Ordering::Acquire);
            if !bmp.get_bit(index as _) {
                let cpu = HvCpu::new(registers);

                VIRTUALIZE_BITMAP.store(*bmp.clone().set_bit(index as _, true), Ordering::Release);

                let cpu = self.cpus.push_mut(cpu);

                cpu.hvfs.logger.info(LogEvent::VirtualizingProcessor(index));

                // unwrap rocks
                cpu.prepare(index).unwrap();
                cpu.virtualize().unwrap();
            }
        });
    }
}
