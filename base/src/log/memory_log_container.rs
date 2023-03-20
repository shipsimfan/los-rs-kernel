use super::message::LogMessage;
use alloc::vec::Vec;

pub(super) struct MemoryLogContainer {
    messages: [Option<LogMessage>; MAX_PRE_HEAP_LOG_MESSAGES],
    num_messages: usize,
    heap_message: Vec<LogMessage>,
}

const MAX_PRE_HEAP_LOG_MESSAGES: usize = 8;

impl MemoryLogContainer {
    pub(super) const fn new() -> Self {
        MemoryLogContainer {
            messages: [None, None, None, None, None, None, None, None],
            num_messages: 0,
            heap_message: Vec::new(),
        }
    }

    pub(super) fn log(&mut self, message: LogMessage) {
        if self.num_messages < MAX_PRE_HEAP_LOG_MESSAGES {
            self.messages[self.num_messages] = Some(message);
        } else {
            self.heap_message.push(message);
        }

        self.num_messages += 1;
    }
}
