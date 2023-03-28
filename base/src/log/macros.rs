#[macro_export]
macro_rules! log {
    ($logger: expr, $level: expr, $str: literal) => {
        $logger.log($level, $str.into())
    };

    ($logger:expr, $level:expr, $($arg:tt)+) => (
        $logger.log($level, alloc::format!("{}", format_args!($($arg)*)).into()))
}

#[macro_export]
macro_rules! log_fatal {
    ($logger: expr, $str: literal) => {
        $crate::log!($logger, $crate::Level::Fatal, $str)
    };
    ($logger: expr, $($arg:tt)+) => {
        $crate::log!($logger, $crate::Level::Fatal, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger: expr, $str: literal) => {
        $crate::log!($logger, $crate::Level::Error, $str)
    };
    ($logger: expr, $($arg:tt)+) => {
        $crate::log!($logger, $crate::Level::Error, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger: expr, $str: literal) => {
        $crate::log!($logger, $crate::Level::Warning, $str)
    };
    ($logger: expr, $($arg:tt)+) => {
        $crate::log!($logger, $crate::Level::Warning, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger: expr, $str: literal) => {
        $crate::log!($logger, $crate::Level::Info, $str)
    };
    ($logger: expr, $($arg:tt)+) => {
        $crate::log!($logger, $crate::Level::Info, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger: expr, $str: literal) => {
        $crate::log!($logger, $crate::Level::Debug, $str)
    };
    ($logger: expr, $($arg:tt)+) => {
        $crate::log!($logger, $crate::Level::Debug, $($arg)+)
    };
}
