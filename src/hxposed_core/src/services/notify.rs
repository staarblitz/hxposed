use crate::services::process::HxProcess;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Eq, PartialEq, Debug)]
pub enum ProcessState {
    Created,
    Terminated,
}

pub type ProcessEventHandler = Box<dyn FnMut(HxProcess, u32, ProcessState) -> u32>;

pub struct HxNotifyServices {
    events: Vec<ProcessEventHandler>,
}

impl HxNotifyServices {}
