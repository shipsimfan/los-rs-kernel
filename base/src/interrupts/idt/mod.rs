use self::idtr::IDTR;
use core::arch::global_asm;
use vector::Vector;

mod constants;
mod idtr;
mod vector;

pub(crate) use constants::*;

#[repr(packed)]
pub(crate) struct IDT {
    vectors: [Vector; NUM_VECTORS],
}

global_asm!(include_str!("idt.asm"));

impl IDT {
    pub(super) const fn null() -> Self {
        IDT {
            vectors: [Vector::null(); NUM_VECTORS],
        }
    }

    pub(crate) fn initialize(&self) {
        IDTR::load_idt(self);
    }

    pub(crate) fn set_vector(&mut self, vector: usize, handler: u64) {
        assert!(vector < NUM_VECTORS);
        self.vectors[vector] = Vector::new(handler);
    }
}
