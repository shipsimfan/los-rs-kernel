use core::ops::{Index, IndexMut};
use process::SignalHandleReturn;

#[derive(Clone, Copy)]
pub enum SignalHandler {
    Terminate,
    Ignore,
    Userspace,
}

#[derive(Copy)]
pub struct Signal {
    handler: SignalHandler,
    mask: bool,
    pending: bool,
}

#[derive(Clone)]
pub struct Signals {
    signals: [Signal; 256],
    userspace_handler: usize,
}

#[repr(u8)]
pub enum SignalType {
    Kill = 0,
    Terminate = 1,
    Abort = 2,
    Interrupt = 3,
    Alarm = 4,
}

#[repr(packed(1))]
#[repr(C)]
pub struct UserspaceSignalContext {
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
    pub rflags: u64,
    pub rip: u64,
}

impl Signals {
    pub fn new() -> Self {
        let mut signals = Signals {
            signals: [Signal::new(); 256],
            userspace_handler: 0,
        };

        signals[SignalType::Kill].handler = SignalHandler::Terminate;
        signals[SignalType::Terminate].handler = SignalHandler::Terminate;
        signals[SignalType::Abort].handler = SignalHandler::Terminate;
        signals[SignalType::Interrupt].handler = SignalHandler::Terminate;
        signals[SignalType::Alarm].handler = SignalHandler::Ignore;

        signals
    }

    pub fn raise(&mut self, signal: u8) {
        if !self[signal].mask {
            self[signal].pending = true;
        }
    }

    pub fn set_handler(&mut self, signal: u8, handler: SignalHandler) {
        self[signal].handler = handler;
    }

    pub fn set_userspace_handler(&mut self, handler: usize) {
        self.userspace_handler = handler;
    }

    pub fn mask(&mut self, signal: u8, mask: bool) {
        if signal != SignalType::Kill as u8 {
            self[signal].mask = mask;
        }
    }
}

impl Index<SignalType> for Signals {
    type Output = Signal;

    fn index(&self, index: SignalType) -> &Self::Output {
        &self.signals[index as usize]
    }
}

impl Index<u8> for Signals {
    type Output = Signal;

    fn index(&self, index: u8) -> &Self::Output {
        &self.signals[index as usize]
    }
}

impl IndexMut<SignalType> for Signals {
    fn index_mut(&mut self, index: SignalType) -> &mut Self::Output {
        &mut self.signals[index as usize]
    }
}

impl IndexMut<u8> for Signals {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.signals[index as usize]
    }
}

impl process::Signals for Signals {
    type UserspaceContext = UserspaceSignalContext;

    fn new() -> Self {
        Self::new()
    }

    fn handle(&mut self, userspace_context: (Self::UserspaceContext, u64)) -> SignalHandleReturn {
        for i in 0..=255 {
            if self[i].pending {
                match self[i].handler {
                    SignalHandler::Ignore => self[i].pending = false,
                    SignalHandler::Terminate => {
                        self[i].pending = false;
                        return SignalHandleReturn::Kill(128 + i as isize);
                    }
                    SignalHandler::Userspace => {
                        self[i].pending = false;

                        unsafe {
                            let (context, rsp) = userspace_context;

                            // Build the context on the userspace stack
                            let stack: *mut UserspaceSignalContext = (rsp
                                - core::mem::size_of::<UserspaceSignalContext>() as u64)
                                as *mut _;

                            *stack = context;

                            // Handle signal
                            return SignalHandleReturn::Userspace(
                                stack as u64,
                                self.userspace_handler,
                                i as u64,
                            );
                        }
                    }
                }
            }
        }

        SignalHandleReturn::None
    }
}

impl Signal {
    pub fn new() -> Self {
        Signal {
            handler: SignalHandler::Ignore,
            mask: false,
            pending: false,
        }
    }
}

impl Clone for Signal {
    fn clone(&self) -> Self {
        Signal {
            handler: self.handler,
            pending: false,
            mask: self.mask,
        }
    }
}

impl SignalHandler {
    pub fn from(value: usize) -> Option<SignalHandler> {
        match value {
            0 => Some(SignalHandler::Terminate),
            1 => Some(SignalHandler::Ignore),
            2 => Some(SignalHandler::Userspace),
            _ => None,
        }
    }
}
