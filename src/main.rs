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
#![feature(trait_alias)]

mod bootloader;
mod device;
mod error;
mod event;
mod filesystem;
mod interrupts;
mod locks;
mod logger;
mod map;
mod memory;
mod process;
mod queue;
mod session;
mod syscall;
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
    interrupts::initialize_system_calls();
    unsafe { memory::initialize(mmap, gmode) };
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

    log!("Creating kinit process . . . ");
    process::create_process(kinit, None);
    logln!("OK!");
    process::yield_thread();

    loop {}
}

fn kinit() -> isize {
    logln!("Loading filesystem drivers . . .");
    filesystem::register_filesystem_driver(filesystem::drivers::fat32::detect_fat32_filesystem);

    logln!("Loading boot device drivers . . . ");
    device::drivers::hpet::initialize();
    device::drivers::pci::initialize();
    device::drivers::ide::initialize();
    device::drivers::cmos::initialize();

    if device::get_device("/boot_video").is_ok() {
        logln!("Starting boot video session . . . ");
        //logger::disable_boot_video_logging();
        match session::create_console_session("/boot_video") {
            Ok(_) => {}
            Err(status) => panic!("Failed to create boot video session: {}", status),
        }
    }

    logln!("Loading device drivers . . . ");
    device::drivers::ps2::initialize();

    process::exit_process(0);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    match info.message() {
        Some(msg) => {
            logln!("Fatal Error: {}", msg);
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
