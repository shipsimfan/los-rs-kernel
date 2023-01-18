use crate::Event;
use alloc::string::String;

pub trait Formatter {
    fn format(&self, event: Event) -> String;
}
