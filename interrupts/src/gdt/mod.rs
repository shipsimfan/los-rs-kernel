use core::cell::RefCell;

mod segment;
mod tss;

pub use tss::TSS;

#[repr(packed)]
pub struct GDT<'a> {
    entries: [segment::Descriptor; 7],
    tss: &'a RefCell<TSS>,
}

const TSS_ENTRIES: (segment::Descriptor, segment::Descriptor) = segment::Descriptor::new_tss();
const TSS_OFFSET: usize = 5;

const INITIAL_ENTRIES: [segment::Descriptor; 7] = [
    segment::Descriptor::new_null(),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring0, segment::SegmentType::Code),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring0, segment::SegmentType::Data),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring3, segment::SegmentType::Code),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring3, segment::SegmentType::Data),
    TSS_ENTRIES.0,
    TSS_ENTRIES.1,
];

impl<'a> GDT<'a> {
    pub fn new(tss: &'a RefCell<TSS>) -> Self {
        let mut gdt = GDT {
            entries: INITIAL_ENTRIES,
            tss,
        };

        let tss = tss.as_ptr() as u64;
        gdt.entries[TSS_OFFSET + 0].update_tss_low((tss & 0xFFFFFFFF) as u32);
        gdt.entries[TSS_OFFSET + 1].update_tss_high((tss >> 32) as u32);

        gdt
    }

    pub fn set_interrupt_stack(&self, stack: u64) {
        self.tss.borrow_mut().set_interrupt_stack(stack);
    }

    pub fn set_active(&self) {
        panic!("TODO: Implement");
    }
}
