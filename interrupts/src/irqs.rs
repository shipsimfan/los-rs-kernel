use super::Registers;
use base::critical::CriticalLock;

#[derive(Debug, Clone, Copy)]
struct HandlerWithContext {
    handler: Handler,
    context: usize,
}

#[repr(packed(1))]
#[repr(C)]
pub struct IRQInfo {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

pub type Handler = unsafe fn(context: usize);
pub type PostIRQHandler = unsafe fn(irq: usize, registers: &Registers, info: &IRQInfo);

const IRQ_BASE: u8 = 32;

static mut IRQS_INITIALIZED: bool = false;

// POST_IRQ_HANDLER does not require a critical lock because it is set once a boot
static mut POST_IRQ_HANDLER: PostIRQHandler = default_post_irq_handler;

static IRQ_HANDLERS: CriticalLock<[Option<HandlerWithContext>; 16]> = CriticalLock::new([None; 16]);

extern "C" {
    fn irq_handler_0();
    fn irq_handler_1();
    fn irq_handler_2();
    fn irq_handler_3();
    fn irq_handler_4();
    fn irq_handler_5();
    fn irq_handler_6();
    fn irq_handler_7();
    fn irq_handler_8();
    fn irq_handler_9();
    fn irq_handler_10();
    fn irq_handler_11();
    fn irq_handler_12();
    fn irq_handler_13();
    fn irq_handler_14();
    fn irq_handler_15();
}

pub fn initialize(post_irq_handler: PostIRQHandler) {
    unsafe {
        assert!(!IRQS_INITIALIZED);
        IRQS_INITIALIZED = true;

        POST_IRQ_HANDLER = post_irq_handler;
    }

    // Install IRQ handlers
    super::idt::install_interrupt_handler(IRQ_BASE + 0, irq_handler_0 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 1, irq_handler_1 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 2, irq_handler_2 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 3, irq_handler_3 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 4, irq_handler_4 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 5, irq_handler_5 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 6, irq_handler_6 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 7, irq_handler_7 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 8, irq_handler_8 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 9, irq_handler_9 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 10, irq_handler_10 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 11, irq_handler_11 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 12, irq_handler_12 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 13, irq_handler_13 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 14, irq_handler_14 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 15, irq_handler_15 as usize);
}

unsafe fn default_post_irq_handler(_: usize, _: &Registers, _: &IRQInfo) {
    panic!("No default post irq handler setup!");
}

#[no_mangle]
unsafe extern "C" fn common_irq_handler(irq: usize, registers: Registers, info: IRQInfo) {
    let handler = IRQ_HANDLERS.lock()[irq];
    match handler {
        None => {}
        Some(handler) => (handler.handler)(handler.context),
    }

    POST_IRQ_HANDLER(irq, &registers, &info);
}

pub fn install_irq_handler(irq: u8, handler: Handler, context: usize) -> bool {
    if irq > 15 {
        return false;
    }

    let mut irq_handlers = IRQ_HANDLERS.lock();
    match irq_handlers[irq as usize] {
        None => {
            irq_handlers[irq as usize] = Some(HandlerWithContext {
                handler: handler,
                context: context,
            });
            true
        }
        Some(_) => false,
    }
}
