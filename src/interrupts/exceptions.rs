use crate::process;

use super::idt::install_interrupt_handler;

#[repr(packed(1))]
#[repr(C)]
pub struct Registers {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rbp: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
}

#[repr(packed(1))]
#[repr(C)]
pub struct ExceptionInfo {
    pub interrupt: u64,
    pub error_code: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

pub type Handler = unsafe fn(Registers, ExceptionInfo);

extern "C" {
    fn exception_handler_0();
    fn exception_handler_1();
    fn exception_handler_2();
    fn exception_handler_3();
    fn exception_handler_4();
    fn exception_handler_5();
    fn exception_handler_6();
    fn exception_handler_7();
    fn exception_handler_8();
    fn exception_handler_9();
    fn exception_handler_10();
    fn exception_handler_11();
    fn exception_handler_12();
    fn exception_handler_13();
    fn exception_handler_14();
    fn exception_handler_15();
    fn exception_handler_16();
    fn exception_handler_17();
    fn exception_handler_18();
    fn exception_handler_19();
    fn exception_handler_20();
    fn exception_handler_21();
    fn exception_handler_22();
    fn exception_handler_23();
    fn exception_handler_24();
    fn exception_handler_25();
    fn exception_handler_26();
    fn exception_handler_27();
    fn exception_handler_28();
    fn exception_handler_29();
    fn exception_handler_30();
    fn exception_handler_31();
}

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

static mut EXCEPTION_HANDLERS: [Option<Handler>; 32] = [None; 32];

#[no_mangle]
unsafe extern "C" fn common_exception_handler(registers: Registers, info: ExceptionInfo) {
    match EXCEPTION_HANDLERS[info.interrupt as usize] {
        Some(handler) => handler(registers, info),
        None => match process::get_current_thread_option() {
            Some(_) => process::exit_process(129 + (info.interrupt as isize)),
            None => match EXCEPTION_STRINGS.get(info.interrupt as usize) {
                Some(str) => panic!("{} has occurred", str),
                None => {
                    let interrupt = info.interrupt as isize;
                    panic!("Exception handler called on non-exception! ({})", interrupt);
                }
            },
        },
    }

    crate::process::handle_signals(None);
}

pub fn initialize() {
    // Initialize the gdt
    super::gdt::initialize();

    // Initialize the IDT
    super::idt::initialize();

    // Install interrupt handlers
    install_interrupt_handler(0, exception_handler_0 as usize);
    install_interrupt_handler(1, exception_handler_1 as usize);
    install_interrupt_handler(2, exception_handler_2 as usize);
    install_interrupt_handler(3, exception_handler_3 as usize);
    install_interrupt_handler(4, exception_handler_4 as usize);
    install_interrupt_handler(5, exception_handler_5 as usize);
    install_interrupt_handler(6, exception_handler_6 as usize);
    install_interrupt_handler(7, exception_handler_7 as usize);
    install_interrupt_handler(8, exception_handler_8 as usize);
    install_interrupt_handler(9, exception_handler_9 as usize);
    install_interrupt_handler(10, exception_handler_10 as usize);
    install_interrupt_handler(11, exception_handler_11 as usize);
    install_interrupt_handler(12, exception_handler_12 as usize);
    install_interrupt_handler(13, exception_handler_13 as usize);
    install_interrupt_handler(14, exception_handler_14 as usize);
    install_interrupt_handler(15, exception_handler_15 as usize);
    install_interrupt_handler(16, exception_handler_16 as usize);
    install_interrupt_handler(17, exception_handler_17 as usize);
    install_interrupt_handler(18, exception_handler_18 as usize);
    install_interrupt_handler(19, exception_handler_19 as usize);
    install_interrupt_handler(20, exception_handler_20 as usize);
    install_interrupt_handler(21, exception_handler_21 as usize);
    install_interrupt_handler(22, exception_handler_22 as usize);
    install_interrupt_handler(23, exception_handler_23 as usize);
    install_interrupt_handler(24, exception_handler_24 as usize);
    install_interrupt_handler(25, exception_handler_25 as usize);
    install_interrupt_handler(26, exception_handler_26 as usize);
    install_interrupt_handler(27, exception_handler_27 as usize);
    install_interrupt_handler(28, exception_handler_28 as usize);
    install_interrupt_handler(29, exception_handler_29 as usize);
    install_interrupt_handler(30, exception_handler_30 as usize);
    install_interrupt_handler(31, exception_handler_31 as usize);
}

pub fn install_exception_handler(exception: u8, handler: Handler) {
    if exception >= 32 {
        return;
    }

    unsafe { EXCEPTION_HANDLERS[exception as usize] = Some(handler) };
}
