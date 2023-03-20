use alloc::string::String;

#[derive(Clone, Copy)]
pub enum Level {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}

enum Message {
    Static(&'static str),
    Owned(String),
}

#[allow(unused)]
pub(super) struct LogMessage {
    level: Level,
    module: &'static str,
    message: Message,
}

impl LogMessage {
    pub(super) fn new(level: Level, module: &'static str, message: &'static str) -> Self {
        LogMessage {
            level,
            module,
            message: Message::Static(message),
        }
    }
    pub(super) fn new_owned(level: Level, module: &'static str, message: String) -> Self {
        LogMessage {
            level,
            module,
            message: Message::Owned(message),
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
