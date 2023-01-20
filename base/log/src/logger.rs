use crate::{Event, Level, LogController};
use alloc::{string::String, sync::Arc};

pub struct Logger {
    module_name: &'static str,
    log_controller: Arc<LogController>,
}

impl Logger {
    pub fn new(module_name: &'static str, log_controller: Arc<LogController>) -> Self {
        Logger {
            module_name,
            log_controller,
        }
    }

    pub fn log(&self, level: Level, message: String) {
        let event = Event::new(level, self.module_name, message);
        self.log_controller.log(event);
    }
}
