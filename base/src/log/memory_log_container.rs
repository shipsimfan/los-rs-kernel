use super::message::LogMessage;

pub(super) struct MemoryLogContainer {
    messages: [Option<LogMessage>; MAX_PRE_HEAP_LOG_MESSAGES],
    num_messages: usize,
}

const MAX_PRE_HEAP_LOG_MESSAGES: usize = 64;

impl MemoryLogContainer {
    pub(super) const fn new() -> Self {
        MemoryLogContainer {
            messages: [None; MAX_PRE_HEAP_LOG_MESSAGES],
            num_messages: 0,
        }
    }

    pub(super) fn log(&mut self, message: LogMessage) {
        assert!(self.num_messages < MAX_PRE_HEAP_LOG_MESSAGES);

        self.messages[self.num_messages] = Some(message);
        self.num_messages += 1;
    }
}
