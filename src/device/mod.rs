use crate::{error, locks::Mutex};
use alloc::{string::String, vec::Vec};
use core::arch::asm;

pub mod acpi;
pub mod drivers;

mod inner;
mod reference;
mod tree;

pub use inner::*;
pub use reference::*;

static DEVICE_TREE: Mutex<tree::Tree> = Mutex::new(tree::Tree::new());

pub fn register_device(path: &str, device: DeviceReference) -> error::Result<()> {
    DEVICE_TREE.lock().register_device(path, device)
}

pub fn remove_device(path: &str) {
    DEVICE_TREE.lock()._remove_device(path)
}

pub fn get_device(path: &str) -> error::Result<DeviceReference> {
    DEVICE_TREE.lock().get_device(path)
}

pub fn get_children(path: &str) -> error::Result<Vec<String>> {
    DEVICE_TREE.lock().get_children(path)
}

pub fn outb(port: u16, data: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") data) };
}

pub fn _outw(port: u16, data: u16) {
    unsafe { asm!("out dx, ax", in("dx") port, in("ax") data) };
}

pub fn outd(port: u16, data: u32) {
    unsafe { asm!("out dx, eax", in("dx") port, in("eax") data) };
}

pub fn inb(port: u16) -> u8 {
    let ret;
    unsafe { asm!("in al, dx", in("dx") port, out("al") ret) };
    ret
}

pub fn _inw(port: u16) -> u16 {
    let ret;
    unsafe { asm!("in ax, dx", in("dx") port, out("ax") ret) };
    ret
}

pub fn ind(port: u16) -> u32 {
    let ret;
    unsafe { asm!("in eax, dx", in("dx") port, out("eax") ret) };
    ret
}
