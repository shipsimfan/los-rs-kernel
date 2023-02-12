#![no_std]

mod critical;
mod gdt;
mod interrupts;
mod local;

pub use critical::{CriticalKey, CriticalLock, CriticalLockGuard};
pub use gdt::{GDT, TSS};
pub use interrupts::InterruptController;
pub use local::LocalState;
