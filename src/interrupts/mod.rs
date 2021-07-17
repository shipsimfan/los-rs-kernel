pub mod exceptions;
mod gdt;
mod idt;
pub mod irq;

#[repr(packed(1))]
struct CPUPointer {
    _size: u16,
    _ptr: usize,
}

#[no_mangle]
static mut KERNEL_STACK_TOP: usize = 0;

pub fn set_interrupt_stack(stack_pointer: usize) {
    gdt::set_interrupt_stack(stack_pointer);
    unsafe { KERNEL_STACK_TOP = stack_pointer };
}
