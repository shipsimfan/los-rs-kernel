use crate::{Event, Level, LogController};
use alloc::{string::String, sync::Arc};

pub struct Logger<'local> {
    module_name: &'static str,
    log_controller: Arc<LogController<'local>>,
}

impl<'local> Logger<'local> {
    pub fn new(module_name: &'static str, log_controller: Arc<LogController<'local>>) -> Self {
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
