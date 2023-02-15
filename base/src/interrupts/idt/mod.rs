use core::arch::global_asm;
use idtr::IDTR;
use vector::Vector;

mod constants;
mod idtr;
mod vector;

pub(super) use constants::*;

#[repr(packed, C)]
pub(super) struct IDT {
    vectors: [Vector; NUM_VECTORS],
}

global_asm!(include_str!("idt.asm"));

impl IDT {
    pub(super) const fn null() -> Self {
        IDT {
            vectors: [Vector::null(); NUM_VECTORS],
        }
    }

    pub(super) fn initialize(&self) {
        IDTR::load_idt(self);
    }

    pub(super) fn set_vector(&mut self, vector: usize, handler: u64) {
        assert!(vector < NUM_VECTORS);
        self.vectors[vector] = Vector::new(handler);
    }

    pub(super) fn clear_vector(&mut self, vector: usize) {
        assert!(vector < NUM_VECTORS);
        self.vectors[vector] = Vector::null();
    }
}
