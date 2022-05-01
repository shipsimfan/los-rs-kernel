#![no_std]

use alloc::boxed::Box;
use console::UEFIConsole;
use process::{ProcessOwner, Signals};

mod console;
mod font;
mod framebuffer;

extern crate alloc;

static mut UEFI_INITIALIZED: bool = false;

pub fn initialize<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    gmode: &base::bootloader::GraphicsMode,
) {
    unsafe {
        assert!(!UEFI_INITIALIZED);
        UEFI_INITIALIZED = true;
    }

    let console = UEFIConsole::new(gmode);

    device::register_device::<O, D, S>("/uefi", console).expect("Failed to register UEFI device!");

    base::logging::set_logging_output(Some(Box::new(
        device::get_device::<O, D, S>("/uefi").unwrap(),
    )))
}
