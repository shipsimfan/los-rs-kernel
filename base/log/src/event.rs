use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

pub struct Event {
    level: Level,
    module: &'static str,
    message: String,
}

impl Event {
    pub fn new(level: Level, module: &'static str, message: String) -> Self {
        Event {
            level,
            module,
            message,
        }
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn module(&self) -> &str {
        self.module
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Level {
    pub(crate) fn display(&self) -> &'static str {
        match self {
            Level::Debug => "Debug",
            Level::Info => "Info",
            Level::Warning => "Warning",
            Level::Error => "Error",
            Level::Fatal => "Fatal",
        }
    }

    pub(crate) fn display_lower(&self) -> &'static str {
        match self {
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warning => "warning",
            Level::Error => "error",
            Level::Fatal => "fatal",
        }
    }

    pub(crate) fn display_upper(&self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }
}

impl core::fmt::Display for Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.display())
    }
}
