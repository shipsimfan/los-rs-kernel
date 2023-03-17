#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_option)]

mod critical;
mod gdt;
mod interrupts;
mod local;
mod memory;

pub use critical::{CriticalKey, CriticalLock, CriticalLockGuard};
pub use gdt::{GDT, TSS};
pub use interrupts::{ExceptionInfo, ExceptionType, InterruptController};
pub use local::LocalState;
pub use memory::{MemoryDescriptor, MemoryManager, MemoryMap, PhysicalAddress};