#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(const_fn_trait_bound)]

mod bootloader;
mod device;
mod error;
mod interrupts;
mod locks;
mod logger;
mod map;
mod memory;
mod process;
mod queue;
mod session;
mod time;

extern crate alloc;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kmain(
    gmode: *const bootloader::GraphicsMode,
    mmap: *const bootloader::MemoryMap,
    rsdp: *const core::ffi::c_void,
) -> ! {
    interrupts::exceptions::initialize();
    memory::initialize(mmap, gmode);
    device::drivers::uefi::initialize(gmode); // Also initializes the UEFI console as logger output

    logln!("Booting Lance OS version {} . . . ", VERSION);
    logln!(
        "{} / {} MB of RAM available",
        memory::get_free_memory() / 1024 / 1024,
        memory::get_total_memory() / 1024 / 1024
    );
    logln!();

    device::acpi::initialize(rsdp).expect("Failed to initialize ACPI!");

    interrupts::irq::initialize();

    log!("Creating first session . . . ");
    let first_session = session::create_console_session();
    logln!("\x1B2A2]OK\x1B]!");
    log!("Creating startup process . . . ");
    first_session
        .lock()
        .create_process(startup_thread as usize, 0);
    logln!("\x1B2A2]OK\x1B]!");
    process::yield_thread();

    loop {}
}

fn startup_thread() -> usize {
    logln!("Loading device drivers . . . ");

    device::drivers::hpet::initialize();
    device::drivers::pci::initialize();

    logln!("Starting shell . . . ");

    // Must keep one thread alive, then the system may(most likely) crash
    loop {
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log!("\x1BD22]");
    match info.message() {
        Some(msg) => {
            logln!("Fatal Error:\x1B] {}", msg);
            match info.location() {
                Some(location) => logln!("\tLocated at {}", location),
                None => {}
            }
        }
        None => logln!("{}", info),
    }

    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}
