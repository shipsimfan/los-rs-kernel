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

pub fn exit_thread(exit_code: isize) -> ! {
    LocalState::get()
        .process_controller()
        .borrow()
        .current_thread()
        .kill(exit_code);
    ProcessManager::get().r#yield(None);
    panic!("Returned from exit thread yield");
}

pub fn exit_process(exit_code: isize) -> ! {
    LocalState::get()
        .process_controller()
        .borrow()
        .current_thread()
        .process()
        .kill(exit_code);
    ProcessManager::get().r#yield(None);
    panic!("Return from exit process yield");
}
