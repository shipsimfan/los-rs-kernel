use core::arch::asm;

pub struct CriticalState {
    count: usize,
}

// CriticalKey is used to match up critical enters and leaves
pub struct CriticalKey;

impl CriticalState {
    pub fn new() -> Self {
        CriticalState { count: 0 }
    }

    #[inline(always)]
    pub unsafe fn enter(&mut self) -> CriticalKey {
        asm!("cli");
        self.count += 1;
        CriticalKey
    }

    #[inline(always)]
    pub unsafe fn enter_assert(&mut self) -> CriticalKey {
        assert!(self.count == 0);
        self.enter()
    }

    #[inline(always)]
    pub unsafe fn leave(&mut self, key: CriticalKey) {
        self.leave_without_sti(key);
        if self.count == 0 {
            asm!("sti");
        }
    }

    #[inline(always)]
    #[allow(unused_variables)]
    pub unsafe fn leave_without_sti(&mut self, key: CriticalKey) {
        self.count -= 1;
    }
}
