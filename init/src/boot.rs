use core::arch::global_asm;

global_asm!(include_str!("./boot.asm"), options(raw));
