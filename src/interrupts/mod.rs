pub mod exceptions;
mod gdt;
mod idt;
pub mod irq;

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

extern "C" {
    fn init_system_calls();
}

#[no_mangle]
static mut KERNEL_STACK_TOP: usize = 0;

pub fn set_interrupt_stack(stack_pointer: usize) {
    gdt::set_interrupt_stack(stack_pointer);
    unsafe { KERNEL_STACK_TOP = stack_pointer };
}

pub fn initialize_system_calls() {
    unsafe { init_system_calls() };
}
