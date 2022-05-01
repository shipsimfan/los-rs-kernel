use core::arch::asm;

pub fn outb(port: u16, data: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") data) };
}

pub fn outw(port: u16, data: u16) {
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

pub fn inw(port: u16) -> u16 {
    let ret;
    unsafe { asm!("in ax, dx", in("dx") port, out("ax") ret) };
    ret
}

pub fn ind(port: u16) -> u32 {
    let ret;
    unsafe { asm!("in eax, dx", in("dx") port, out("eax") ret) };
    ret
}
