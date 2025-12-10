use alloc::string::String;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Luid {
    pub low: u32,
    pub high: i32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum TokenType {
    Primary,
    Impersonation
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum ImpersonationLevel {
    Anonymous,
    Identification,
    Impersonation,
    Delegation
}

pub struct TokenSource {
    pub name: String,
    pub luid: Luid,
}