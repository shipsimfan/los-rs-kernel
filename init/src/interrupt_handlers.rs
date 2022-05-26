use interrupts::{exceptions::ExceptionInfo, irqs::IRQInfo, Registers};
use process_types::ProcessTypes;

const EXCEPTION_STRINGS: [&str; 32] = [
    "A divide by zero exception",
    "A debug exception",
    "A non-maskable interrupt",
    "A breakpoint",
    "An overflow",
    "A bound range exceeded exception",
    "An invalid opcode exception",
    "A device not available exception",
    "A double fault",
    "A coprocessor segment overrun exception",
    "An invalid TSS exception",
    "A segement not present exception",
    "A stack-segment fault",
    "A general protection fault",
    "A page fault",
    "",
    "An x87 floating-point exception",
    "An alignment check exception",
    "A machine check exception",
    "An SIMD floating-point exception",
    "A virtualization exception",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "A security exception",
    "",
];

pub unsafe fn default_exception_handler(_: &Registers, info: &ExceptionInfo) {
    match EXCEPTION_STRINGS.get(info.interrupt as usize) {
        Some(str) => panic!("{} has occurred", str),
        None => {
            let interrupt = info.interrupt as isize;
            panic!("Exception handler called on non-exception! ({})", interrupt);
        }
    }
}

pub unsafe fn post_exception_handler(registers: &Registers, info: &ExceptionInfo) {
    if info.rip < memory::KERNEL_VMA as u64 {
        let userspace_context = (
            ipc::UserspaceSignalContext {
                r15: registers.r15,
                r14: registers.r14,
                r13: registers.r13,
                r12: registers.r12,
                r11: registers.r11,
                r10: registers.r10,
                r9: registers.r9,
                r8: registers.r8,
                rbp: registers.rbp,
                rdi: registers.rdi,
                rsi: registers.rsi,
                rdx: registers.rdx,
                rcx: registers.rcx,
                rbx: registers.rbx,
                rax: registers.rax,
                rflags: info.rflags,
                rip: info.rip,
            },
            info.rsp,
        );
        process::handle_signals::<ProcessTypes>(userspace_context);
    }

    acpi::end_interrupt();
}

pub unsafe fn post_irq_handler(irq: usize, registers: &Registers, info: &IRQInfo) {
    acpi::end_irq(irq as u8);
    acpi::end_interrupt();

    if info.rip < memory::KERNEL_VMA as u64 {
        let userspace_context = (
            ipc::UserspaceSignalContext {
                r15: registers.r15,
                r14: registers.r14,
                r13: registers.r13,
                r12: registers.r12,
                r11: registers.r11,
                r10: registers.r10,
                r9: registers.r9,
                r8: registers.r8,
                rbp: registers.rbp,
                rdi: registers.rdi,
                rsi: registers.rsi,
                rdx: registers.rdx,
                rcx: registers.rcx,
                rbx: registers.rbx,
                rax: registers.rax,
                rflags: info.rflags,
                rip: info.rip,
            },
            info.rsp,
        );
        base::critical::leave_local_without_sti();
        process::handle_signals::<ProcessTypes>(userspace_context);
        base::critical::enter_local();
    }
}

pub unsafe fn null_access_exception_handler(_: usize) {
    base::critical::leave_local_without_sti();
    process::exit_process::<ProcessTypes>(129);
}

pub unsafe fn invalid_access_exception_handler(_: usize) {
    base::critical::leave_local_without_sti();
    process::exit_process::<ProcessTypes>(130);
}
