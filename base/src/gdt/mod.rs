use crate::local::set_gs;
use core::{arch::global_asm, cell::RefCell};

mod constants;
mod segment;
mod tss;

pub(crate) use constants::KERNEL_CODE_SEGMENT_OFFSET;
pub(self) use constants::*;

pub use tss::TSS;

#[repr(packed, C)]
pub struct GDT<'a> {
    entries: [segment::Descriptor; SEGMENT_COUNT],
    tss: &'a RefCell<TSS>,
}

#[repr(packed, C)]
#[allow(unused)]
struct GDTR<'a> {
    limit: u16,
    address: *const GDT<'a>,
}

global_asm!(include_str!("./gdt.asm"));

extern "C" {
    #[allow(improper_ctypes)]
    fn set_active_gdt(gdt: *const GDTR, kernel_code_segment: u16, kernel_data_segment: u16);
}

impl<'a> GDT<'a> {
    pub fn new(tss: &'a RefCell<TSS>) -> Self {
        let mut gdt = GDT {
            entries: INITIAL_ENTRIES,
            tss,
        };

        let tss = tss.as_ptr() as u64;
        gdt.entries[TSS_INDEX + 0].update_tss_low((tss & 0xFFFFFFFF) as u32);
        gdt.entries[TSS_INDEX + 1].update_tss_high((tss >> 32) as u32);

        gdt
    }

    pub fn set_interrupt_stack(&self, stack: u64) {
        self.tss.borrow_mut().set_interrupt_stack(stack);
    }

    pub fn set_active(&self, null_gs_ptr: usize) {
        unsafe {
            set_active_gdt(
                &GDTR {
                    limit: core::mem::size_of::<[segment::Descriptor; SEGMENT_COUNT]>() as u16,
                    address: self,
                },
                KERNEL_CODE_SEGMENT_OFFSET as u16,
                KERNEL_DATA_SEGMENT_OFFSET as u16,
            );
            set_gs(null_gs_ptr);
        }
    }
}
