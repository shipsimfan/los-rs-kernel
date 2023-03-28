use alloc::borrow::Cow;

#[derive(Clone, Copy)]
pub enum Level {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}

#[allow(unused)]
pub(super) struct LogMessage {
    level: Level,
    module: Cow<'static, str>,
    message: Cow<'static, str>,
}

impl LogMessage {
    pub(super) fn new(level: Level, module: Cow<'static, str>, message: Cow<'static, str>) -> Self {
        LogMessage {
            level,
            module,
            message,
        }
    }
}

impl core::fmt::Display for Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Level::Fatal => "Fatal",
                Level::Error => "Error",
                Level::Warning => "Warning",
                Level::Info => "Info",
                Level::Debug => "Debug",
            }
        )
    }
}
