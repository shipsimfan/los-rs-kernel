mod output;

use alloc::format;
use core::fmt;

pub use output::{set_logging_output, LogEvent, LogLevel, LogOutput, LogOutputMut};

#[doc(hidden)]
#[inline(always)]
pub fn _log(module: &'static str, level: LogLevel, args: fmt::Arguments) {
    match unsafe { &output::CURRENT_LOG_OUTPUT } {
        Some(output) => output.log(LogEvent::new(module, format!("{}", args), level)),
        None => {}
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => (
        base::logging::_log(crate::MODULE_NAME, $level, format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => (base::logging::_log(crate::MODULE_NAME, $crate::logging::LogLevel::FatalError, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => (base::logging::_log(crate::MODULE_NAME, $crate::logging::LogLevel::Error, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => (base::logging::_log(crate::MODULE_NAME, $crate::logging::LogLevel::Warning, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => (base::logging::_log(crate::MODULE_NAME, $crate::logging::LogLevel::Information, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => (base::logging::_log(crate::MODULE_NAME, $crate::logging::LogLevel::Debugging, format_args!($($arg)*)));
}
