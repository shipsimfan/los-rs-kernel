use crate::{Level, LogController};
use alloc::string::String;

pub struct Logger {
    name: &'static str,
}

impl Logger {
    pub const fn new(name: &'static str) -> Self {
        Logger { name }
    }

    pub fn log(&self, level: Level, message: &'static str) {
        LogController::get().log(level, self.name, message);
    }

    pub fn log_owned(&self, level: Level, message: String) {
        LogController::get().log_owned(level, self.name, message);
    }
}
