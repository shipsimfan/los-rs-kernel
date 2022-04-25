#![no_std]

mod gdt;
mod idt;

pub mod exceptions;
pub mod irqs;
pub mod system_calls;

extern crate alloc;

#[repr(packed(1))]
struct CPUPointer {
    _size: u16,
    _ptr: usize,
}

#[repr(packed(1))]
#[repr(C)]
pub struct Registers {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
}

static mut INTERRUPTS_INITIALIZED: bool = false;

pub fn initialize(
    default_exception_handler: exceptions::Handler,
    post_all_exception_handler: exceptions::Handler,
    post_irq_handler: irqs::PostIRQHandler,
    system_call_handler: system_calls::Handler,
) {
    unsafe {
        assert!(!INTERRUPTS_INITIALIZED);
        INTERRUPTS_INITIALIZED = true;
    }

    gdt::initialize();

    idt::initialize();

    exceptions::initialize(default_exception_handler, post_all_exception_handler);

    irqs::initialize(post_irq_handler);

    system_calls::initialize(system_call_handler);
}

pub fn set_interrupt_stack(stack_pointer: usize) {
    gdt::set_interrupt_stack(stack_pointer);
}
