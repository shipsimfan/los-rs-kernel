use crate::local::set_gs;
use alloc::boxed::Box;
use core::{arch::global_asm, cell::RefCell, pin::Pin};
use tss::TSS;

mod constants;
mod segment;
mod tss;

pub(crate) use constants::KERNEL_CODE_SEGMENT_OFFSET;
pub(self) use constants::*;

#[repr(C)]
pub(crate) struct GDT {
    entries: [segment::Descriptor; SEGMENT_COUNT],
    tss: Pin<Box<RefCell<TSS>>>,
}

#[repr(packed, C)]
#[allow(unused)]
struct GDTR {
    limit: u16,
    address: *const [segment::Descriptor],
}

global_asm!(include_str!("./gdt.asm"));

extern "C" {
    #[allow(improper_ctypes)]
    fn set_active_gdt(
        gdt: *const GDTR,
        kernel_code_segment: u16,
        kernel_data_segment: u16,
        reload_cs_fn: usize,
    );
    fn reload_cs();
}

impl GDT {
    pub(crate) fn new() -> Pin<Box<Self>> {
        let tss = TSS::new();
        let tss_address = tss.as_ptr() as usize;

        let mut entries = INITIAL_ENTRIES;
        entries[TSS_INDEX + 0].update_tss_low((tss_address & 0xFFFFFFFF) as u32);
        entries[TSS_INDEX + 1].update_tss_high((tss_address >> 32) as u32);

        Box::pin(GDT { entries, tss })
    }

    pub(crate) fn set_interrupt_stack(&self, stack: u64) {
        self.tss.borrow_mut().set_interrupt_stack(stack);
    }

    pub(crate) fn set_active(&self, null_gs_ptr: usize) {
        unsafe {
            set_active_gdt(
                &GDTR {
                    limit: core::mem::size_of::<[segment::Descriptor; SEGMENT_COUNT]>() as u16,
                    address: &self.entries,
                },
                KERNEL_CODE_SEGMENT_OFFSET as u16,
                KERNEL_DATA_SEGMENT_OFFSET as u16,
                reload_cs as usize,
            );
            set_gs(null_gs_ptr);
        }
    }
}
