use super::{
    memory_log_container::MemoryLogContainer, message::LogMessage, output::LogOutput, Level,
};
use crate::CriticalLock;
use alloc::string::String;

enum Output {
    Static(&'static dyn LogOutput),
}

pub struct LogController {
    // TODO: Change critical lock to mutex
    output: CriticalLock<Option<Output>>,

    memory_log_container: CriticalLock<MemoryLogContainer>,
}

static LOG_CONTROLLER: LogController = LogController::new();

impl LogController {
    pub const fn new() -> Self {
        LogController {
            output: CriticalLock::new(None),
            memory_log_container: CriticalLock::new(MemoryLogContainer::new()),
        }
    }

    pub fn get() -> &'static LogController {
        &LOG_CONTROLLER
    }

    pub fn set_static_output(&self, output: &'static dyn LogOutput) {
        *self.output.lock() = Some(Output::Static(output));
    }

    pub(super) fn log(&self, level: Level, module: &'static str, message: &'static str) {
        match self.output.lock().as_ref().map(|output| match output {
            Output::Static(output) => output,
        }) {
            Some(output) => writeln!(output, "[{}][{}] {}", level, module, message),
            None => {}
        }

        let message = LogMessage::new(level, module, message);
        self.memory_log_container.lock().log(message);
    }

    pub(super) fn log_owned(&self, level: Level, module: &'static str, message: String) {
        match self.output.lock().as_ref().map(|output| match output {
            Output::Static(output) => output,
        }) {
            Some(output) => writeln!(output, "[{}][{}] {}", level, module, message),
            None => {}
        }

        let message = LogMessage::new_owned(level, module, message);
        self.memory_log_container.lock().log(message);
    }
}
