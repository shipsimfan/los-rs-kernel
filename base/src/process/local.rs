use super::{Thread, ThreadQueue};

pub(crate) struct LocalProcessController {
    current_thread: Option<Thread>,

    next_thread: Option<Thread>,
    current_thread_target_queue: Option<ThreadQueue>,
}

impl LocalProcessController {
    pub(crate) fn new() -> Self {
        LocalProcessController {
            current_thread: None,

            next_thread: None,
            current_thread_target_queue: None,
        }
    }
}
