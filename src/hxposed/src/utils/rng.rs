// not really RNG but does the job
pub struct SimpleCounter {
    pub state: u32,
}

unsafe impl Sync for SimpleCounter {}
unsafe impl Send for SimpleCounter {}

impl SimpleCounter {
    pub fn next_u32(&mut self) -> u32 {
        let x = self.state;
        self.state = x + 1;
        x
    }
}
