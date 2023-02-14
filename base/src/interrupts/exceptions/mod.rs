use super::idt::{IDT, NUM_EXCEPTIONS};
use core::{arch::global_asm, ops::Index};
use handlers::HANDLERS;

mod common_handler;
mod handlers;
mod info;
mod types;

pub use info::ExceptionInfo;
pub use types::ExceptionType;

pub type ExceptionHandler = fn(ExceptionInfo);

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

    pub(super) fn set_exception(&mut self, exception: ExceptionType, handler: ExceptionHandler) {
        assert!(self.exceptions[exception as usize].is_none());
        self.exceptions[exception as usize] = Some(handler);
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

impl Index<u64> for Exceptions {
    type Output = Option<ExceptionHandler>;

    fn index(&self, index: u64) -> &Self::Output {
        &self.exceptions[index as usize]
    }
}
