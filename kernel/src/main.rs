#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use base::{critical::CriticalLock, log_fatal, log_info};
use core::arch::asm;
use memory::Heap;

mod interrupt_handlers;
mod system_calls;

const MODULE_NAME: &str = "Kernel";

#[global_allocator]
static HEAP: CriticalLock<Heap> = CriticalLock::new(Heap);

#[no_mangle]
pub extern "C" fn kmain(
    gmode: *const base::bootloader::GraphicsMode,
    mmap: *const base::bootloader::MemoryMap,
    _rsdp: *const core::ffi::c_void,
) -> ! {
    // Convert passed pointers into references
    let gmode = unsafe { &*gmode };
    let mmap = unsafe { &*mmap };

    // Initialize interrupts
    interrupts::initialize(
        interrupt_handlers::default_exception_handler,
        interrupt_handlers::post_exception_handler,
        interrupt_handlers::post_irq_handler,
        system_calls::handler,
    );

    // Initialize memory
    memory::initialize(
        mmap,
        gmode,
        interrupt_handlers::null_access_exception_handler,
        interrupt_handlers::invalid_access_exception_handler,
    );

    log_info!("Booting Lance Operating System . . .");

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    match info.message() {
        Some(msg) => {
            log_fatal!("{}", msg);
            match info.location() {
                Some(location) => log_fatal!("\tLocated at {}", location),
                None => {}
            }
        }
        None => log_fatal!("{}", info),
    }

    loop {
        unsafe { asm!("cli; hlt") };
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    // Cannot print or panic as that requires allocations
    loop {
        unsafe { asm!("cli; hlt") };
    }
}
