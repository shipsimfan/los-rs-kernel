use super::{exceptions::Exceptions, idt::IDT};
use crate::{CriticalLock, ExceptionHandler, ExceptionType};

pub struct InterruptController {
    idt: IDT,
    exceptions: Exceptions,

    initialized: bool,
}

static CONTROLLER: CriticalLock<InterruptController> =
    CriticalLock::new(InterruptController::null());

impl InterruptController {
    pub fn get() -> &'static CriticalLock<InterruptController> {
        &CONTROLLER
    }

    pub(self) const fn null() -> Self {
        InterruptController {
            idt: IDT::null(),
            exceptions: Exceptions::null(),
            initialized: false,
        }
    }

    pub(super) fn exceptions(&self) -> &Exceptions {
        &self.exceptions
    }

    pub fn set_exception(&mut self, exception: ExceptionType, handler: ExceptionHandler) {
        self.exceptions.set_exception(exception, handler)
    }

    pub fn initialize(&mut self) {
        assert!(!self.initialized);
        self.initialized = true;

        self.idt.initialize();
        self.exceptions.initialize(&mut self.idt);
    }
}
