pub mod exceptions;
mod gdt;
mod idt;
pub mod irq;

#[repr(packed(1))]
struct CPUPointer {
    _size: u16,
    _ptr: usize,
}

pub fn set_interrupt_stack(stack_pointer: usize) {
    gdt::set_interrupt_stack(stack_pointer);
}
