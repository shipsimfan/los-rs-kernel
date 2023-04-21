#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_option)]
#![feature(allocator_api)]
#![feature(int_roundings)]
#![feature(pointer_byte_offsets)]
#![feature(const_trait_impl)]
#![feature(associated_type_defaults)]

extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};

mod boot_video;
mod critical;
mod error;
mod gdt;
mod interrupts;
mod local;
mod log;
mod memory;
mod sync;
mod util;

pub mod process;

pub use boot_video::BootVideo;
pub use critical::{CriticalKey, CriticalLock, CriticalLockGuard, CriticalRefCell};
pub use error::{Error, StandardError};
pub use gdt::{GDT, TSS};
pub use interrupts::{ExceptionInfo, ExceptionType, InterruptController};
pub use local::LocalState;
pub use log::{Level, LogController, LogOutput, Logger};
pub use memory::{
    AddressSpace, MemoryDescriptor, MemoryManager, MemoryMap, PhysicalAddress, SlabAllocator,
    KERNEL_VMA,
};
pub use process::{Process, ProcessManager, Thread, ThreadQueue};
pub use sync::{Mutex, MutexGuard};
pub use util::{Increment, Map, Mappable, MappableMut, Queue};

static INIITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn initialize<M: MemoryMap, B: BootVideo>(memory_map: M, boot_video: &CriticalLock<B>) {
    assert!(!INIITIALIZED.swap(true, Ordering::AcqRel));

    let logger = Logger::from("Base");
    log_info!(logger, "Initializing");

    // Initialize IDT
    let interrupt_controller = InterruptController::get();
    interrupt_controller.lock().initialize();

    // Initialize the memory manager
    let memory_manager = MemoryManager::get();
    let framebuffer_memory = boot_video.lock().framebuffer_memory();
    memory_manager.initialize(memory_map, framebuffer_memory);

    log_info!(logger, "Initialization complete");
}
