use crate::{LocalState, ProcessManager};

pub fn spawn_kernel_thread(entry: fn(usize) -> isize, context: usize) {
    let new_thread = LocalState::get()
        .process_controller()
        .borrow()
        .current_thread()
        .process_arc()
        .create_thread(entry as usize, context);
    ProcessManager::get().queue_thread(new_thread);
}
