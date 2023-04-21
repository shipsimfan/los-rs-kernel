use crate::{LocalState, Process, ProcessManager, Thread};
use alloc::{borrow::Cow, sync::Arc};

pub fn spawn_kernel_thread(entry: fn(usize) -> isize, context: usize) -> u64 {
    let new_thread = LocalState::get()
        .process_controller()
        .borrow()
        .current_thread()
        .process_arc()
        .create_thread(entry as usize, context);
    let id = new_thread.id();
    ProcessManager::get().queue_thread(new_thread);
    id
}

pub fn spawn_kernel_process<S: Into<Cow<'static, str>>>(
    entry: fn(usize) -> isize,
    context: usize,
    name: S,
) -> Arc<Process> {
    ProcessManager::get().create_process(entry as usize, context, name)
}

pub fn get_process(id: u64) -> Option<Arc<Process>> {
    ProcessManager::get().get_process(id)
}

pub fn get_thread<F, T>(id: u64, f: F) -> Option<T>
where
    F: FnOnce(&Thread) -> T,
{
    let process = LocalState::get()
        .process_controller()
        .borrow()
        .current_thread()
        .process_arc()
        .clone();

    process.get_thread(id, f)
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
