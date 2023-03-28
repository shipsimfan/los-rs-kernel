#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_option)]
#![feature(allocator_api)]
#![feature(int_roundings)]
#![feature(pointer_byte_offsets)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(const_trait_impl)]

extern crate alloc;

mod boot_video;
mod critical;
mod gdt;
mod interrupts;
mod local;
mod log;
mod memory;
mod sync;

pub use boot_video::BootVideo;
pub use critical::{CriticalKey, CriticalLock, CriticalLockGuard};
pub use gdt::{GDT, TSS};
pub use interrupts::{ExceptionInfo, ExceptionType, InterruptController};
pub use local::LocalState;
pub use log::{Level, LogController, LogOutput, Logger};
pub use memory::{MemoryDescriptor, MemoryManager, MemoryMap, PhysicalAddress, SlabAllocator};
pub use sync::{Mutex, MutexGuard};
