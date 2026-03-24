pub mod call;
pub mod error;
pub mod func;
pub mod requests;
pub mod responses;
pub mod status;
pub mod utils;

pub type HxObject = u64;
pub type Handle = u64;
pub type ProcessObject = HxObject;
pub type ThreadObject = HxObject;
pub type TokenObject = HxObject;
pub type RmdObject = HxObject;
pub type CallbackObject = HxObject;
pub type AsyncCookie = HxObject;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ObjectType {
    Process(ProcessObject),
    Thread(ThreadObject),
    Token(TokenObject),
    Rmd(RmdObject),
    Registry(u64),
    Unknown
}

impl ObjectType {
    pub fn into_raw(self) -> (u64, u64) {
        match self {
            ObjectType::Process(p) => (1, p),
            ObjectType::Thread(t) => (2, t),
            ObjectType::Token(t) => (3, t),
            ObjectType::Rmd(m) => (4, m),
            ObjectType::Registry(r) => (5, r),
            ObjectType::Unknown => (0, 0),
        }
    }

    pub fn from_raw(object: u64, value: u64) -> ObjectType {
        match object {
            1 => ObjectType::Process(value),
            2 => ObjectType::Thread(value),
            3 => ObjectType::Token(value),
            4 => ObjectType::Rmd(value),
            5 => ObjectType::Registry(value),
            _ => ObjectType::Unknown,
        }
    }
}

impl Into<u64> for ObjectType {
    fn into(self) -> u64 {
        match self {
            ObjectType::Process(x) => x,
            ObjectType::Thread(x) => x,
            ObjectType::Token(x) => x,
            ObjectType::Rmd(x) => x,
            ObjectType::Registry(x) => x,
            ObjectType::Unknown => 0,
        }
    }
}