use core::arch::x86_64;

pub struct CpuBenchmark {
    tsc: u64,
}

impl CpuBenchmark {
    pub fn begin() -> Self {
        Self {
            tsc: unsafe { x86_64::_rdtsc() },
        }
    }

    /// Returns how many cycles passed
    pub fn end(self) -> u64 {
        let end_tsc = unsafe { x86_64::_rdtsc() };

        end_tsc - self.tsc
    }
}
