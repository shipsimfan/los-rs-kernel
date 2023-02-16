#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]

use base::{LocalState, GDT, TSS};
use core::arch::asm;
use global_state::GlobalState;

const MODULE_NAME: &str = "Kernel";

mod boot;

#[no_mangle]
pub extern "C" fn kmain(
    gmode: *const uefi::raw::GraphicsMode,
    memory_map: *const uefi::raw::MemoryMap,
    rsdp: *const core::ffi::c_void,
) -> ! {
    // Create the GDT
    let tss = TSS::new();
    let gdt = GDT::new(&tss);
    gdt.set_active();

    // Create the local state
    let local_state_container = LocalState::new(&gdt);
    let local_state = local_state_container.set_active();

    // Create the global state
    GlobalState::initialize::<uefi::MemoryMap>(memory_map.into());

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
