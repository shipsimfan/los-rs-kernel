use crate::{controller::LogControllerOwner, Event, Level};
use alloc::string::String;
use core::marker::PhantomData;

pub struct Logger<G: LogControllerOwner> {
    module_name: &'static str,
    phantom: PhantomData<G>,
}

impl<G: LogControllerOwner> Logger<G> {
    pub fn new(module_name: &'static str) -> Self {
        Logger {
            module_name,
            phantom: PhantomData,
        }
    }

    pub fn log(&self, level: Level, message: String) {
        let event = Event::new(level, self.module_name, message);

        //LocalState::get::<G>().global().log_controller().lock_read().log(event);
    }
}
