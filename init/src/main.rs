#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]

use base::{CriticalLock, LocalState, LogController, GDT, TSS};
use core::{arch::asm, ffi::c_void, fmt::Write, ptr::NonNull};
use global_state::GlobalState;
use uefi::UEFIBootVideo;

mod boot;

static BOOT_VIDEO: CriticalLock<UEFIBootVideo> = CriticalLock::new(UEFIBootVideo::null());

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn kmain(
    graphics_mode: NonNull<uefi::raw::GraphicsMode>,
    memory_map: NonNull<uefi::raw::MemoryMap>,
    rsdp: NonNull<core::ffi::c_void>,
    null_gs_ptr: NonNull<c_void>,
) -> ! {
    // Setup boot video
    BOOT_VIDEO.lock().initialize(graphics_mode);
    LogController::get().set_static_output(&BOOT_VIDEO);

    // Create the GDT
    let tss = TSS::new();
    let gdt = GDT::new(&tss);
    gdt.set_active(null_gs_ptr.as_ptr() as usize);

    // Create the global state
    GlobalState::initialize::<uefi::MemoryMap, _>(memory_map.into(), &BOOT_VIDEO);

    // Create the local state
    let mut local_state_container = LocalState::new(&gdt);
    let local_state = local_state_container.set_active();

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut boot_video = BOOT_VIDEO.lock();

    match info.message() {
        Some(msg) => {
            writeln!(boot_video, "{}", msg).ok();
            match info.location() {
                Some(location) => {
                    writeln!(boot_video, "\tLocated at {}", location).ok();
                }
                None => {}
            }
        }
        None => {
            writeln!(boot_video, "{}", info).ok();
        }
    }

    infinite_loop();
}

fn infinite_loop() -> ! {
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
