mod keycode;

pub use keycode::KeyState;
pub use keycode::Keycode;

#[derive(Debug, PartialEq)]
#[repr(usize)]
pub enum Event {
    KeyPress(Keycode, KeyState),
    KeyRelease(Keycode, KeyState),
}

#[repr(C)]
pub struct CEvent {
    class: usize,
    param1: usize,
    param2: usize,
}

impl From<Event> for CEvent {
    fn from(event: Event) -> Self {
        let (class, param1, param2) = match event {
            Event::KeyPress(key, state) => (0, key as usize, state.into()),
            Event::KeyRelease(key, state) => (1, key as usize, state.into()),
        };

        CEvent {
            class,
            param1,
            param2,
        }
    }
}
