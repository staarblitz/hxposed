use bit_field::BitField;
use x86::dtables::DescriptorTablePointer;
use x86::segmentation::{SegmentSelector, SystemDescriptorTypes64};

pub struct SegmentDescriptor {
    low64: SegmentDescriptorRaw,
    upper_base: Option<u32>,
}

impl SegmentDescriptor {
    pub(crate) fn try_from_gdtr(
        gdtr: &DescriptorTablePointer<u64>,
        selector: SegmentSelector,
    ) -> Result<Self, ()> {
        if selector.contains(SegmentSelector::TI_LDT) {
            return Err(());
        }

        let index = selector.index() as usize;
        if index == 0 {
            return Err(());
        }

        let gdt = unsafe {
            core::slice::from_raw_parts(gdtr.base.cast::<u64>(), usize::from(gdtr.limit + 1) / 8)
        };

        let raw = match gdt.get(index) {
            Some(raw) => raw,
            None => {
                return Err(());
            }
        };

        let low64 = SegmentDescriptorRaw::from(*raw);
        let upper_base = if low64.is_16byte() {
            let index: usize = index + 1;

            let raw = match gdt.get(index) {
                Some(raw) => raw,
                None => {
                    return Err(());
                }
            };

            let Ok(upper_base) = u32::try_from(*raw) else {
                return Err(());
            };

            Some(upper_base)
        } else {
            None
        };
        Ok(Self { low64, upper_base })
    }

    pub(crate) fn base(&self) -> u64 {
        if let Some(upper_base) = self.upper_base {
            self.low64.base() as u64 | (u64::from(upper_base) << 32)
        } else {
            self.low64.base() as _
        }
    }
}

struct SegmentDescriptorRaw {
    raw: u64,
}

impl SegmentDescriptorRaw {
    fn is_16byte(&self) -> bool {
        let high32 = self.raw.get_bits(32..);
        let system = high32.get_bit(12); // descriptor type
        let type_ = high32.get_bits(8..=11) as u8;
        !system
            && (type_ == SystemDescriptorTypes64::TssAvailable as u8
                || type_ == SystemDescriptorTypes64::TssBusy as u8)
    }

    fn base(&self) -> u32 {
        let low32 = self.raw.get_bits(..=31);
        let high32 = self.raw.get_bits(32..);

        let base_high = high32.get_bits(24..=31) << 24;
        let base_middle = high32.get_bits(0..=7) << 16;
        let base_low = low32.get_bits(16..=31);
        u32::try_from(base_high | base_middle | base_low).unwrap()
    }
}

impl From<u64> for SegmentDescriptorRaw {
    fn from(raw: u64) -> Self {
        Self { raw }
    }
}
