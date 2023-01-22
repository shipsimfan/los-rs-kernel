use core::{arch::asm, cell::RefCell};

pub struct CriticalState {
    count: RefCell<usize>,
}

// CriticalKey is used to match up critical enters and leaves
pub struct CriticalKey;

impl CriticalState {
    pub fn new() -> Self {
        CriticalState {
            count: RefCell::new(0),
        }
    }

    #[inline(always)]
    pub unsafe fn enter(&self) -> CriticalKey {
        asm!("cli");
        *self.count.borrow_mut() += 1;
        CriticalKey
    }

    #[inline(always)]
    pub unsafe fn enter_assert(&self) -> CriticalKey {
        assert!(*self.count.borrow() == 0);
        self.enter()
    }

    #[inline(always)]
    pub unsafe fn leave(&self, key: CriticalKey) {
        self.leave_without_sti(key);
        if *self.count.borrow() == 0 {
            asm!("sti");
        }
    }

    #[inline(always)]
    #[allow(unused_variables)]
    pub unsafe fn leave_without_sti(&self, key: CriticalKey) {
        *self.count.borrow_mut() -= 1;
    }
}
