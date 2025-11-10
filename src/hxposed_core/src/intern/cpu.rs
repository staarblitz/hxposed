use x86::cpuid::VendorInfo;

pub enum CpuVendor {
    AMD,
    Intel,
    Unknown,
}

impl CpuVendor {
    pub fn get_vendor() -> Self {
        let id = x86::cpuid::CpuId::new();
        let info = id.get_vendor_info().unwrap();
        let string = info.as_str();

        if string.starts_with("GenuineIntel") {
            Self::Intel
        } else if string.starts_with("AuthenticAMD") {
            Self::AMD
        } else {
            Self::Unknown
        }
    }
}
