use super::segment;

// SEGMENT INDEXES
pub(super) const KERNEL_CODE_SEGMENT_INDEX: usize = 1;
pub(super) const KERNEL_DATA_SEGMENT_INDEX: usize = 2;
pub(super) const TSS_INDEX: usize = 5;
pub(super) const SEGMENT_COUNT: usize = 7;

// SEGMENT OFFSETS
pub(crate) const KERNEL_CODE_SEGMENT_OFFSET: usize =
    core::mem::size_of::<segment::Descriptor>() * KERNEL_CODE_SEGMENT_INDEX;
pub(super) const KERNEL_DATA_SEGMENT_OFFSET: usize =
    core::mem::size_of::<segment::Descriptor>() * KERNEL_DATA_SEGMENT_INDEX;

// SEGEMTNS
pub(super) const TSS_ENTRIES: (segment::Descriptor, segment::Descriptor) =
    segment::Descriptor::new_tss();

pub(super) const INITIAL_ENTRIES: [segment::Descriptor; SEGMENT_COUNT] = [
    segment::Descriptor::new_null(),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring0, segment::SegmentType::Code),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring0, segment::SegmentType::Data),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring3, segment::SegmentType::Code),
    segment::Descriptor::new_normal(segment::PrivilegeLevel::Ring3, segment::SegmentType::Data),
    TSS_ENTRIES.0,
    TSS_ENTRIES.1,
];
