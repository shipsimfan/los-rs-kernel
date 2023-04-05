use crate::{Level, LogController};
use alloc::{borrow::Cow, string::String};

pub struct Logger {
    name: Cow<'static, str>,
    minimum: Level,
}

impl Logger {
    pub const fn new(name: Cow<'static, str>) -> Self {
        Logger {
            name,
            minimum: Level::Debug,
        }
    }

    pub fn set_minimum_level(&mut self, level: Level) {
        self.minimum = level;
    }

    pub fn log(&self, level: Level, message: Cow<'static, str>) {
        if level < self.minimum {
            return;
        }

        LogController::get().log(level, self.name.clone(), message);
    }
}

impl From<String> for Logger {
    fn from(value: String) -> Self {
        Self::new(value.into())
    }
}

impl const From<&'static str> for Logger {
    fn from(value: &'static str) -> Self {
        Self::new(Cow::Borrowed(value))
    }
}
