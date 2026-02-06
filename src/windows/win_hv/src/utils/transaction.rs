use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Transaction {
    rollbacks: Vec<Box<dyn FnOnce()>>,
    committed: bool,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            rollbacks: Vec::new(),
            committed: false,
        }
    }

    pub fn enlist<F>(&mut self, op: F)
    where
        F: FnOnce() + 'static,
    {
        self.rollbacks.push(Box::new(op));
    }

    pub fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if !self.committed {
            for op in self.rollbacks.drain(..).rev() {
                op();
            }
        }
    }
}
