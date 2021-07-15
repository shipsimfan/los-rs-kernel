mod keycode;

pub use keycode::Keycode;

#[derive(Debug, PartialEq)]
#[repr(usize)]
pub enum Event {
    KeyPress(Keycode),
    KeyRelease(Keycode),
}

#[repr(C)]
pub struct CEvent {
    class: usize,
    param1: usize,
}

impl From<Event> for CEvent {
    fn from(event: Event) -> Self {
        let (class, param1) = match event {
            Event::KeyPress(key) => (0, key as usize),
            Event::KeyRelease(key) => (1, key as usize),
        };

        CEvent { class, param1 }
    }
}
