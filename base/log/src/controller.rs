use crate::{Event, Formatter, Level, LogOutput};
use alloc::{boxed::Box, sync::Arc};

pub struct LogController {
    output: Option<Arc<dyn LogOutput>>,
    minimum_level: Level,
    formatter: Box<dyn Formatter>,
}

pub trait LogControllerOwner {
    fn log_controller(&self) -> &LogController; // TODO: Return an &RWLock<LogController>
}

#[cfg(debug_assertions)]
const DEFAULT_MINIMUM_LEVEL: Level = Level::Debug;

#[cfg(not(debug_assertions))]
const DEFAULT_MINIMUM_LEVEL: Level = Level::Warning;

impl LogController {
    pub fn new(formatter: Box<dyn Formatter>) -> Self {
        // TODO: Return an RWLock<Self>
        LogController {
            output: None,
            minimum_level: DEFAULT_MINIMUM_LEVEL,
            formatter,
        }
    }

    pub fn log(&self, event: Event) {
        if event.level() >= self.minimum_level {
            self.output
                .as_ref()
                .map(|output| output.write(&self.formatter.format(event)));
        }
    }

    pub fn set_output<O: LogOutput>(&mut self, output: Arc<O>) {
        self.output = Some(output);
    }

    pub fn clear_output(&mut self) {
        self.output = None;
    }

    pub fn set_minimum_level(&mut self, minimum_level: Level) {
        self.minimum_level = minimum_level;
    }
}
