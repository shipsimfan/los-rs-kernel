use crate::CriticalLock;
use exceptions::Exceptions;
use idt::IDT;

mod exceptions;
mod idt;

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

    pub fn initialize(&mut self) {
        assert!(!self.initialized);
        self.initialized = true;

        self.idt.initialize();
        self.exceptions.initialize(&mut self.idt);
    }
}
