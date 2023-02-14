use crate::interrupts::{IRQInfo, Registers};

#[repr(packed, C)]
pub struct ExceptionInfo {
    registers: Registers,
    interrupt: u64,
    error_code: u64,
    irq: IRQInfo,
}

impl ExceptionInfo {
    pub fn interrupt(&self) -> u64 {
        self.interrupt
    }
}
