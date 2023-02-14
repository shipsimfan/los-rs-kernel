use crate::interrupts::idt::FIRST_IRQ;

use super::{idt::NUM_IRQS, IRQInfo};

pub type IRQ = extern "C" fn(info: IRQInfo);

pub(super) struct IRQs {
    irqs: [Option<IRQ>; NUM_IRQS],
    lowest_free: Option<usize>,
}

impl IRQs {
    pub(super) const fn null() -> Self {
        IRQs {
            irqs: [None; NUM_IRQS],
            lowest_free: Some(0),
        }
    }

    pub(super) fn allocate(&mut self, irq: IRQ) -> usize {
        match self.lowest_free {
            Some(lowest_free) => {
                self.irqs[lowest_free] = Some(irq);
                self.find_lowest_free();
                lowest_free + FIRST_IRQ
            }
            None => panic!("Attempting to allocate an IRQ with none remaining"),
        }
    }

    pub(super) fn free(&mut self, irq: usize) {
        let irq = irq - FIRST_IRQ;

        assert!(self.irqs[irq].is_some());
        self.irqs[irq] = None;

        match self.lowest_free {
            Some(lowest_free) => match irq < lowest_free {
                true => self.lowest_free = Some(irq),
                false => {}
            },
            None => self.lowest_free = Some(irq),
        }
    }

    fn find_lowest_free(&mut self) {
        let lowest_free = self.lowest_free.unwrap();

        for i in lowest_free..NUM_IRQS {
            if self.irqs[i].is_none() {
                self.lowest_free = Some(i);
                return;
            }
        }

        self.lowest_free = None;
    }
}
