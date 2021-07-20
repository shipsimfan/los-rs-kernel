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

use alloc::string::ToString;

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
    let first_session =
        session::create_console_session("/uefi_console").expect("Failed to create first session");
    logln!("\x1B2A2]OK\x1B]!");
    log!("Creating startup process . . . ");
    first_session
        .lock()
        .create_process(startup_thread as usize, 0, None);
    logln!("\x1B2A2]OK\x1B]!");
    process::yield_thread();

    loop {}
}

fn startup_thread() -> usize {
    log!("Loading filesystem drivers . . .");
    filesystem::register_filesystem_driver(filesystem::drivers::fat32::detect_fat32_filesystem);
    logln!("\x1B2A2]OK\x1B]!");

    logln!("Loading device drivers . . . ");

    device::drivers::hpet::initialize();
    device::drivers::pci::initialize();
    device::drivers::ide::initialize();
    device::drivers::ps2::initialize();

    logln!("Starting shell . . . ");

    match process::get_current_thread_mut()
        .get_process_mut()
        .get_session_mut()
    {
        None => panic!("Starting process is a daemon!"),
        Some(session) => match session.get_sub_session_mut() {
            session::SubSession::Console(console) => match console.clear() {
                Ok(()) => {}
                Err(status) => panic!("Unable to clear console! {}", status),
            },
        },
    }

    let pid = match process::execute(
        ":1/los/bin/shell.app",
        alloc::vec::Vec::new(),
        alloc::vec!["PATH=:1/los/bin".to_string()],
    ) {
        Ok(pid) => pid,
        Err(status) => {
            logln!("Error while starting shell: {}", status);
            loop {
                unsafe { asm!("hlt") }
            }
        }
    };
    let status = process::wait_process(pid);
    logln!("Shell exited with status: {:#X}", status);

    loop {
        unsafe { asm!("sti;hlt") };
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
