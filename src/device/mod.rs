use crate::{error, locks::Mutex};
use alloc::{boxed::Box, sync::Arc};
use core::arch::asm;

pub mod acpi;
pub mod drivers;
mod tree;

pub trait Device: Send {
    fn read(&self, address: usize, buffer: &mut [u8]) -> error::Result<()>;
    fn write(&mut self, address: usize, buffer: &[u8]) -> error::Result<()>;

    fn read_register(&mut self, address: usize) -> error::Result<usize>;
    fn write_register(&mut self, address: usize, value: usize) -> error::Result<()>;

    fn ioctrl(&mut self, code: usize, argument: usize) -> error::Result<usize>;
}

pub type DeviceBox = Arc<Mutex<Box<dyn Device>>>;

static DEVICE_TREE: Mutex<tree::Tree> = Mutex::new(tree::Tree::new());

pub fn register_device(path: &str, device: DeviceBox) -> error::Result<()> {
    DEVICE_TREE.lock().register_device(path, device)
}

pub fn remove_device(path: &str) {
    DEVICE_TREE.lock()._remove_device(path)
}

pub fn get_device(path: &str) -> error::Result<DeviceBox> {
    DEVICE_TREE.lock().get_device(path)
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
