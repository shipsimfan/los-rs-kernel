#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_option)]
#![feature(allocator_api)]
#![feature(int_roundings)]
#![feature(pointer_byte_offsets)]
#![feature(nonnull_slice_from_raw_parts)]

mod critical;
mod gdt;
mod interrupts;
mod local;
mod memory;

pub use critical::{CriticalKey, CriticalLock, CriticalLockGuard};
pub use gdt::{GDT, TSS};
pub use interrupts::{ExceptionInfo, ExceptionType, InterruptController};
pub use local::LocalState;
pub use memory::{MemoryDescriptor, MemoryManager, MemoryMap, PhysicalAddress, SlabAllocator};
