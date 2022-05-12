#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::borrow::ToOwned;
use base::{critical::CriticalLock, log_debug, log_fatal, log_info};
use core::arch::asm;
use memory::Heap;

mod interrupt_handlers;
mod process_types;
mod system_calls;

const MODULE_NAME: &str = "Kernel";

#[global_allocator]
static HEAP: CriticalLock<Heap> = CriticalLock::new(Heap);

#[no_mangle]
pub extern "C" fn kmain(
    gmode: *const base::bootloader::GraphicsMode,
    mmap: *const base::bootloader::MemoryMap,
    rsdp: *const core::ffi::c_void,
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

    // Initialize process manager
    process::initialize::<process_types::ProcessTypes>();

    // Initialize device manager
    device::initialize::<process_types::ProcessTypes>();

    // Initialize boot video
    uefi::initialize::<process_types::ProcessTypes>(gmode);

    log_info!("Booting Lance Operating System . . .");
    let memory_usage = memory::get_memory_usage();
    log_info!(
        "{} / {} MB of RAM available",
        memory_usage.free_memory() / 1024 / 1024,
        memory_usage.available_memory() / 1024 / 1024
    );

    // Initialize ACPI
    acpi::initialize::<process_types::ProcessTypes>(rsdp);

    // Launch kinit process
    log_info!("Starting kinit process . . .");
    process::create_process::<process_types::ProcessTypes>(
        kinit,
        0,
        process_types::TempDescriptors,
        "kinit".to_owned(),
        false,
    );
    process::yield_thread::<process_types::ProcessTypes>(None);

    loop {}
}

fn test(_: usize) -> isize {
    loop {
        log_debug!("Test Loop");

        process::queue_and_yield::<process_types::ProcessTypes>();
    }
}

fn kinit(_: usize) -> isize {
    log_info!("kinit running!");

    sessions::initialize::<process_types::ProcessTypes>();

    let thread = process::create_thread::<process_types::ProcessTypes>(test, 0);

    process::queue_and_yield::<process_types::ProcessTypes>();

    log_debug!("Killed other thread: {}", !thread.alive());

    process::kill_thread(&thread, 100);

    log_debug!("Killed other thread: {}", !thread.alive());

    log_info!("Starting initial session . . .");
    sessions::create_console_session::<process_types::ProcessTypes>(
        device::get_device("/boot_video").unwrap(),
    )
    .unwrap();
    log_info!("Initial session started!");

    loop {
        unsafe { asm!("sti; hlt") };
    }
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
