#![no_std]

use alloc::boxed::Box;
use base::log_info;
use console::UEFIConsole;
use process::ProcessTypes;

mod console;
mod font;
mod framebuffer;

extern crate alloc;

const MODULE_NAME: &str = "UEFI";

static mut UEFI_INITIALIZED: bool = false;

pub fn initialize<T: ProcessTypes + 'static>(gmode: &base::bootloader::GraphicsMode) {
    log_info!("Initializing . . . ");

    unsafe {
        assert!(!UEFI_INITIALIZED);
        UEFI_INITIALIZED = true;
    }

    let console = UEFIConsole::new(gmode);

    device::register_device::<T>("/boot_video", console).expect("Failed to register UEFI device!");

    base::logging::set_logging_output(Some(Box::new(
        device::get_device::<T>("/boot_video").unwrap(),
    )));

    log_info!("Initialized!");
}
