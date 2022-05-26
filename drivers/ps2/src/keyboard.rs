use super::controller;
use alloc::boxed::Box;
use base::{error::PS2_DRIVER_MODULE_NUMBER, multi_owner::Owner};
use device::{inb, outb, Device};
use process::{Mutex, ProcessTypes};
use sessions::{Event, KeyState, Keycode, Session};

#[derive(Debug)]
enum KeyboardError {
    NotSupported,
    InvalidIOCtrl,
}

pub struct Keyboard<T: ProcessTypes<Owner = Box<dyn Session<T>>>> {
    session: Owner<Box<dyn Session<T>>>,
    key_state: KeyState,
    ignore_next_irq: bool,
}

impl<T: ProcessTypes<Owner = Box<dyn Session<T>>>> Keyboard<T> {
    pub fn new(
        controller: &mut controller::Controller<T>,
        port: usize,
        session: Owner<Box<dyn Session<T>>>,
    ) -> base::error::Result<Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>> {
        // Set scancode set to 1
        controller.write_and_wait(port, 0xF0)?;

        outb(controller::REGISTER_DATA, 1);

        // Enable sdevicecanning
        controller.write_and_wait(port, controller::DEVICE_COMMAND_ENABLE_SCAN)?;
        controller.stop_initializing(port);

        // Return keyboard
        Ok(Owner::new(Box::new(Keyboard {
            session,
            key_state: KeyState::new(),
            ignore_next_irq: false,
        }) as Box<dyn Device>))
    }

    fn scancode_to_event(&mut self, scancode: u8) -> Event {
        if scancode == 0xE0 {
            self.ignore_next_irq = true;

            let scancode = inb(controller::REGISTER_DATA);

            let (key_press, scancode) = if scancode > 0x80 {
                (false, scancode - 0x80)
            } else {
                (true, scancode)
            };

            let key = match scancode {
                0x1C => Keycode::Enter,
                0x1D => {
                    self.key_state.right_ctrl = key_press;
                    Keycode::RightControl
                }
                0x35 => Keycode::ForwardSlash,
                0x38 => {
                    self.key_state.right_alt = key_press;
                    Keycode::RightAlt
                }
                0x47 => Keycode::Home,
                0x48 => Keycode::UpArrow,
                0x49 => Keycode::PageUp,
                0x4B => Keycode::LeftArrow,
                0x4D => Keycode::RightArrow,
                0x4F => Keycode::End,
                0x50 => Keycode::DownArrow,
                0x51 => Keycode::PageDown,
                0x52 => Keycode::Insert,
                0x53 => Keycode::Delete,
                _ => Keycode::Undefined,
            };

            if key_press {
                return Event::KeyPress(key, self.key_state);
            } else {
                return Event::KeyRelease(key, self.key_state);
            }
        }

        if scancode < 0x58 {
            if SCANCODES[scancode as usize] == Keycode::CapsLock {
                self.key_state.caps_lock = !self.key_state.caps_lock;
            } else if SCANCODES[scancode as usize] == Keycode::NumLock {
                self.key_state.num_lock = !self.key_state.num_lock;
            } else if SCANCODES[scancode as usize] == Keycode::ScrollLock {
                self.key_state.scroll_lock = !self.key_state.scroll_lock;
            }
        }

        let (key_press, scancode) = if scancode > 0x80 {
            (false, scancode - 0x80)
        } else {
            (true, scancode)
        };

        let key = if scancode > 0x58 {
            Keycode::Undefined
        } else {
            SCANCODES[scancode as usize]
        };

        if key == Keycode::LeftShift {
            self.key_state.left_shift = key_press;
        } else if key == Keycode::RightShift {
            self.key_state.right_shift = key_press;
        } else if key == Keycode::LeftControl {
            self.key_state.left_ctrl = key_press;
        } else if key == Keycode::RightControl {
            self.key_state.right_ctrl = key_press;
        } else if key == Keycode::LeftAlt {
            self.key_state.left_alt = key_press;
        } else if key == Keycode::RightAlt {
            self.key_state.right_alt = key_press;
        }

        if key_press {
            Event::KeyPress(key, self.key_state)
        } else {
            Event::KeyRelease(key, self.key_state)
        }
    }

    fn irq(&mut self, data: u8) {
        if self.ignore_next_irq {
            self.ignore_next_irq = false;
        } else {
            let event = self.scancode_to_event(data);
            self.session.lock(|session| session.push_event(event));
        }
    }
}

impl<T: ProcessTypes<Owner = Box<dyn Session<T>>>> Device for Keyboard<T> {
    fn read(&self, _: usize, _: &mut [u8]) -> base::error::Result<usize> {
        Err(Box::new(KeyboardError::NotSupported))
    }

    fn write(&mut self, _: usize, _: &[u8]) -> base::error::Result<usize> {
        Err(Box::new(KeyboardError::NotSupported))
    }

    fn read_register(&mut self, _: usize) -> base::error::Result<usize> {
        Err(Box::new(KeyboardError::NotSupported))
    }

    fn write_register(&mut self, _: usize, _: usize) -> base::error::Result<()> {
        Err(Box::new(KeyboardError::NotSupported))
    }

    fn ioctrl(&mut self, code: usize, argument: usize) -> base::error::Result<usize> {
        match code {
            0 => {
                self.irq(argument as u8);
                Ok(0)
            }
            _ => Err(Box::new(KeyboardError::InvalidIOCtrl)),
        }
    }
}

impl base::error::Error for KeyboardError {
    fn module_number(&self) -> i32 {
        PS2_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            KeyboardError::NotSupported => base::error::Status::NotSupported,
            KeyboardError::InvalidIOCtrl => base::error::Status::InvalidIOCtrl,
        }
    }
}

impl core::fmt::Display for KeyboardError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KeyboardError::NotSupported => write!(f, "Not supported for PS/2 keyboard"),
            KeyboardError::InvalidIOCtrl => write!(f, "Invalid I/O control for PS/2 keyboard"),
        }
    }
}

const SCANCODES: [Keycode; 89] = [
    Keycode::Undefined,
    Keycode::Escape,
    Keycode::One,
    Keycode::Two,
    Keycode::Three,
    Keycode::Four,
    Keycode::Five,
    Keycode::Six,
    Keycode::Seven,
    Keycode::Eight,
    Keycode::Nine,
    Keycode::Zero,
    Keycode::Minus,
    Keycode::Equal,
    Keycode::Backspace,
    Keycode::Tab,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::R,
    Keycode::T,
    Keycode::Y,
    Keycode::U,
    Keycode::I,
    Keycode::O,
    Keycode::P,
    Keycode::OpenSquareBracket,
    Keycode::CloseSquareBracket,
    Keycode::Enter,
    Keycode::LeftControl,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::F,
    Keycode::G,
    Keycode::H,
    Keycode::J,
    Keycode::K,
    Keycode::L,
    Keycode::SemiColon,
    Keycode::Quote,
    Keycode::Tick,
    Keycode::LeftShift,
    Keycode::Backslash,
    Keycode::Z,
    Keycode::X,
    Keycode::C,
    Keycode::V,
    Keycode::B,
    Keycode::N,
    Keycode::M,
    Keycode::Comma,
    Keycode::Period,
    Keycode::ForwardSlash,
    Keycode::RightShift,
    Keycode::NumAsterick,
    Keycode::LeftAlt,
    Keycode::Space,
    Keycode::CapsLock,
    Keycode::F1,
    Keycode::F2,
    Keycode::F3,
    Keycode::F4,
    Keycode::F5,
    Keycode::F6,
    Keycode::F7,
    Keycode::F8,
    Keycode::F9,
    Keycode::F10,
    Keycode::NumLock,
    Keycode::ScrollLock,
    Keycode::Seven,
    Keycode::Eight,
    Keycode::Nine,
    Keycode::NumMinus,
    Keycode::Four,
    Keycode::Five,
    Keycode::Six,
    Keycode::NumPlus,
    Keycode::One,
    Keycode::Two,
    Keycode::Three,
    Keycode::Zero,
    Keycode::NumPeriod,
    Keycode::Undefined,
    Keycode::Undefined,
    Keycode::Undefined,
    Keycode::F11,
    Keycode::F12,
];
