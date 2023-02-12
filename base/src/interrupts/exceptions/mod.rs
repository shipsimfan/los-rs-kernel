use super::idt::{IDT, NUM_EXCEPTIONS};
use core::arch::global_asm;
use handlers::HANDLERS;

mod common_handler;
mod handlers;

pub type ExceptionHandler = fn();

pub(super) struct Exceptions {
    initialized: bool,
    exceptions: [Option<ExceptionHandler>; NUM_EXCEPTIONS],
}

global_asm!(include_str!("exceptions.asm"));

impl Exceptions {
    pub(super) const fn null() -> Self {
        Exceptions {
            initialized: false,
            exceptions: [None; NUM_EXCEPTIONS],
        }
    }

    pub(super) fn initialize(&mut self, idt: &mut IDT) {
        assert!(!self.initialized);
        self.initialized = true;

        assert!(HANDLERS.len() >= NUM_EXCEPTIONS);
        for i in 0..HANDLERS.len() {
            idt.set_vector(i, HANDLERS[i] as u64);
        }
    }
}
