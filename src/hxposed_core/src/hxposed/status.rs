use core::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default, Debug)]
pub enum HypervisorStatus {
    #[default]
    Unknown,
    SystemVirtualized,
    SystemDeVirtualized,
}

impl From<u32> for HypervisorStatus {
    fn from(value: u32) -> Self {
        match value {
            1 => HypervisorStatus::SystemVirtualized,
            2 => HypervisorStatus::SystemDeVirtualized,
            _ => HypervisorStatus::Unknown,
        }
    }
}

impl Into<u32> for HypervisorStatus {
    fn into(self) -> u32 {
        match self {
            HypervisorStatus::SystemVirtualized => 1,
            HypervisorStatus::SystemDeVirtualized => 2,
            HypervisorStatus::Unknown => 0,
        }
    }
}

impl Display for HypervisorStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            HypervisorStatus::Unknown => write!(f, "Unknown"),
            HypervisorStatus::SystemVirtualized => write!(f, "SystemVirtualized"),
            HypervisorStatus::SystemDeVirtualized => write!(f, "SystemDeVirtualized"),
        }
    }
}
