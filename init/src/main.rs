#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]

use core::arch::asm;

const MODULE_NAME: &str = "Kernel";

mod boot;

#[no_mangle]
pub extern "C" fn kmain(
    gmode: *const bootloader::GraphicsMode,
    mmap: *const bootloader::MemoryMap,
    rsdp: *const core::ffi::c_void,
) -> ! {
    // Convert passed pointers into references
    let gmode = unsafe { &*gmode };
    let mmap = unsafe { &*mmap };

    // Create the GDT
    let tss = interrupts::TSS::new();
    let gdt = interrupts::GDT::new(&tss);
    gdt.set_active();

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    match info.message() {
        Some(msg) => {
            //log_fatal!("{}", msg);
            match info.location() {
                Some(location) => {} //log_fatal!("\tLocated at {}", location),
                None => {}
            }
        }
        None => {} //log_fatal!("{}", info),
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
