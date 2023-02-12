#![no_std]
use core::{arch::global_asm, cell::RefCell};

mod segment;
mod tss;

pub use tss::TSS;

#[repr(packed)]
pub struct GDT<'a> {
    entries: [segment::Descriptor; SEGMENT_COUNT],
    tss: &'a RefCell<TSS>,
}

#[repr(packed)]
#[allow(unused)]
struct GDTR<'a> {
    limit: u16,
    address: *const GDT<'a>,
}

const TSS_ENTRIES: (segment::Descriptor, segment::Descriptor) = segment::Descriptor::new_tss();
const TSS_INDEX: usize = 5;

pub const KERNEL_CODE_SEGMENT_OFFSET: usize =
    core::mem::size_of::<segment::Descriptor>() * KERNEL_CODE_SEGMENT_INDEX;
const KERNEL_CODE_SEGMENT_INDEX: usize = 1;
const KERNEL_DATA_SEGMENT_OFFSET: usize =
    core::mem::size_of::<segment::Descriptor>() * KERNEL_DATA_SEGMENT_INDEX;
const KERNEL_DATA_SEGMENT_INDEX: usize = 2;
const SEGMENT_COUNT: usize = 7;

const INITIAL_ENTRIES: [segment::Descriptor; SEGMENT_COUNT] = [
    segment::Descriptor::new_null(),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring0, segment::SegmentType::Code),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring0, segment::SegmentType::Data),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring3, segment::SegmentType::Code),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring3, segment::SegmentType::Data),
    TSS_ENTRIES.0,
    TSS_ENTRIES.1,
];

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

    pub fn set_active(&self) {
        unsafe {
            set_active_gdt(
                &GDTR {
                    limit: core::mem::size_of::<[segment::Descriptor; SEGMENT_COUNT]>() as u16,
                    address: self,
                },
                KERNEL_CODE_SEGMENT_OFFSET as u16,
                KERNEL_DATA_SEGMENT_OFFSET as u16,
            )
        }
    }
}
