use alloc::{boxed::Box, string::String};

#[derive(Clone, Copy)]
pub enum LogLevel {
    FatalError,
    Error,
    Warning,
    Information,
    Debugging,
}

pub struct LogEvent {
    module: &'static str,
    message: String,
    level: LogLevel,
}

pub trait LogOutput {
    fn log(&self, event: LogEvent);
}

pub static mut CURRENT_LOG_OUTPUT: Option<Box<dyn LogOutput>> = None;

#[inline(always)]
pub fn set_logging_output(output: Option<Box<dyn LogOutput>>) {
    unsafe { CURRENT_LOG_OUTPUT = output };
}

impl LogEvent {
    #[inline(always)]
    pub const fn new(module: &'static str, message: String, level: LogLevel) -> Self {
        LogEvent {
            module,
            message,
            level,
        }
    }

    #[inline(always)]
    pub fn module(&self) -> &'static str {
        self.module
    }

    #[inline(always)]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[inline(always)]
    pub fn level(&self) -> LogLevel {
        self.level
    }
}
