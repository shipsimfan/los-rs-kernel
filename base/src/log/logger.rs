use crate::{Level, LogController};
use alloc::{borrow::Cow, string::String};

pub struct Logger {
    name: Cow<'static, str>,
}

impl Logger {
    pub const fn new(name: Cow<'static, str>) -> Self {
        Logger { name }
    }

    pub fn log(&self, level: Level, message: Cow<'static, str>) {
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
