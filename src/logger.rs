use crate::locks::Mutex;
use alloc::boxed::Box;
use core::fmt;

pub trait LoggerOutput {}

static LOGGER: Mutex<Option<Box<dyn Send + fmt::Write>>> = Mutex::new(None);

pub fn set_logger(new_logger: Box<dyn Send + fmt::Write>) {
    (*LOGGER.lock()) = Some(new_logger);
}

#[doc(hidden)]
pub fn _log(args: fmt::Arguments) {
    let mut lock = LOGGER.lock();
    match &mut *lock {
        None => {}
        Some(logger) => logger.write_fmt(args).unwrap(),
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::logger::_log(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! logln {
    () => ($crate::log!("\n"));
    ($($arg:tt)*) => ($crate::log!("{}\n", format_args!($($arg)*)));
}
