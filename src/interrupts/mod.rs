pub mod exceptions;
mod gdt;
mod idt;
pub mod irq;

#[repr(packed(1))]
struct CPUPointer {
    _size: u16,
    _ptr: usize,
}

extern "C" {
    fn system_call_handler();
}

#[no_mangle]
static mut KERNEL_STACK_TOP: usize = 0;

pub fn set_interrupt_stack(stack_pointer: usize) {
    gdt::set_interrupt_stack(stack_pointer);
    unsafe { KERNEL_STACK_TOP = stack_pointer };
}

pub fn initialize_system_calls() {
    idt::install_interrupt_handler(0x80, system_call_handler as usize);
}
