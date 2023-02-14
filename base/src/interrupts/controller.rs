use super::{
    exceptions::{ExceptionHandler, Exceptions},
    idt::IDT,
    irqs::{IRQs, IRQ},
};
use crate::{CriticalLock, ExceptionType};

pub struct InterruptController {
    idt: IDT,
    exceptions: Exceptions,
    irqs: IRQs,

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
            irqs: IRQs::null(),

            initialized: false,
        }
    }

    pub(super) fn exceptions(&self) -> &Exceptions {
        &self.exceptions
    }

    pub fn initialize(&mut self) {
        assert!(!self.initialized);
        self.initialized = true;

        self.idt.initialize();
        self.exceptions.initialize(&mut self.idt);
    }

    pub fn set_exception(&mut self, exception: ExceptionType, handler: ExceptionHandler) {
        self.exceptions.set(exception, handler);
    }

    pub fn clear_exception(&mut self, exception: ExceptionType) {
        self.exceptions.clear(exception);
    }

    pub fn allocate_irq(&mut self, irq: IRQ) -> usize {
        let vector = self.irqs.allocate(irq);
        self.idt.set_vector(vector, irq as u64);
        vector
    }

    pub fn free_irq(&mut self, irq: usize) {
        self.irqs.free(irq);
        self.idt.clear_vector(irq);
    }
}
