#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::borrow::ToOwned;
use base::{critical::CriticalLock, log_debug, log_fatal, log_info};
use core::arch::asm;
use memory::Heap;
use process::ProcessTypes;

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

    // Initialize Time
    time::initialize::<process_types::ProcessTypes>();

    // Initialize System Timer
    hpet::initialize::<process_types::ProcessTypes>();

    // Launch kinit process
    log_info!("Starting kinit process . . .");
    process::create_process::<process_types::ProcessTypes>(
        kinit::<process_types::ProcessTypes>,
        0,
        process_types::TempDescriptors,
        "kinit".to_owned(),
        false,
    );
    process::yield_thread::<process_types::ProcessTypes>(None);

    loop {}
}

process::static_generic!(process::Mutex<usize, T>, test_lock);

fn test<T: ProcessTypes + 'static>(_: usize) -> isize {
    let mut lock = test_lock::get::<T>().lock();

    *lock = 69;

    loop {
        log_debug!("Test Loop");

        time::sleep::<T>(500);
    }
}

fn kinit<T: ProcessTypes + 'static>(_: usize) -> isize {
    log_info!("kinit running!");

    test_lock::initialize::<T>(process::Mutex::new(0));

    sessions::initialize::<T>();

    let thread = process::create_thread::<T>(test::<T>, 0);

    process::queue_and_yield::<T>();

    log_info!("Starting initial session . . .");
    sessions::create_console_session::<T>(device::get_device("/boot_video").unwrap()).unwrap();
    log_info!("Initial session started!");

    cmos::initialize();

    time::sleep::<T>(2100);

    log_info!("Killing thread. Alive - {}", thread.alive());

    process::kill_thread(&thread, 69);

    log_info!("Killed thread. Alive - {}", thread.alive());

    log_debug!("Value under lock: {}", *test_lock::get::<T>().lock());

    for i in 0..10 {
        log_debug!("Test: {}", i);
        time::sleep::<T>(250);
    }

    0
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