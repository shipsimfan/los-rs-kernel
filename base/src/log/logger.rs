use crate::{Level, LogController};

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
}
