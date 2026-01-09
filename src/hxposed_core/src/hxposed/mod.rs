pub mod call;
pub mod error;
pub mod func;
pub mod requests;
pub mod responses;
pub mod status;

pub type ProcessObject = u64;
pub type ThreadObject = u64;
pub type TokenObject = u64;
pub type MdlObject = u64;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ObjectType {
    Handle(u64),
    Process(ProcessObject),
    Thread(ThreadObject),
    Token(TokenObject),
    Mdl(MdlObject),
}

impl ObjectType {
    pub fn into_raw(self) -> (u64, u64) {
        match self {
            ObjectType::Handle(h) => (0, h),
            ObjectType::Process(p) => (1, p),
            ObjectType::Thread(t) => (2, t),
            ObjectType::Token(t) => (3, t),
            ObjectType::Mdl(m) => (4, m),
        }
    }

    pub fn from_raw(object: u64, value: u64) -> ObjectType {
        match object {
            0 => ObjectType::Handle(value),
            1 => ObjectType::Process(value),
            2 => ObjectType::Thread(value),
            3 => ObjectType::Token(value),
            4 => ObjectType::Mdl(value),
            _ => panic!("Invalid object id: {}", object),
        }
    }
}

impl Into<u64> for ObjectType {
    fn into(self) -> u64 {
        match self {
            ObjectType::Handle(x) => x,
            ObjectType::Process(x) => x,
            ObjectType::Thread(x) => x,
            ObjectType::Token(x) => x,
            ObjectType::Mdl(x) => x,
        }
    }
}