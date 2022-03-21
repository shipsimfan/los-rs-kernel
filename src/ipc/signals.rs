use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub enum SignalHandler {
    Terminate,
    Ignore,
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
}

#[repr(u8)]
pub enum SignalType {
    Kill = 0,
    Terminate = 1,
    Abort = 2,
    Interrupt = 3,
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

impl Signals {
    pub fn new() -> Self {
        let mut signals = Signals {
            signals: [Signal::new(); 256],
        };

        signals[SignalType::Kill].handler = SignalHandler::Terminate;
        signals[SignalType::Terminate].handler = SignalHandler::Terminate;
        signals[SignalType::Abort].handler = SignalHandler::Terminate;
        signals[SignalType::Interrupt].handler = SignalHandler::Terminate;

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

    pub fn mask(&mut self, signal: u8, mask: bool) {
        if signal != SignalType::Kill as u8 {
            self[signal].mask = mask;
        }
    }

    pub fn handle(&mut self) -> Option<isize> {
        for i in 0..=255 {
            if self[i].pending {
                self[i].pending = false;

                match self[i].handler {
                    SignalHandler::Ignore => {}
                    SignalHandler::Terminate => return Some(128 + i as isize),
                }
            }
        }

        None
    }
}

impl SignalHandler {
    pub fn from(value: usize) -> Option<SignalHandler> {
        match value {
            0 => Some(SignalHandler::Terminate),
            1 => Some(SignalHandler::Ignore),
            _ => None,
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

impl Clone for Signal {
    fn clone(&self) -> Self {
        Signal {
            handler: self.handler,
            pending: false,
            mask: self.mask,
        }
    }
}
