use crate::{LocalState, Process, ProcessManager, StandardError, Thread};
use alloc::{borrow::Cow, sync::Arc};

pub fn get_current_thread<F, T>(f: F) -> T
where
    F: FnOnce(&Thread) -> T,
{
    f(LocalState::get()
        .process_controller()
        .borrow()
        .current_thread())
}

pub fn get_current_thread_opt<F, T>(f: F) -> Option<T>
where
    F: FnOnce(&Thread) -> T,
{
    LocalState::get()
        .process_controller()
        .borrow()
        .current_thread_opt()
        .map(|thread| f(thread))
}

pub fn spawn_kernel_thread(
    process: Option<&Arc<Process>>,
    entry: fn(usize) -> isize,
    context: usize,
) -> u64 {
    let new_thread = match process {
        Some(process) => process.create_thread(entry as usize, context),
        None => {
            get_current_thread(|thread| thread.process_arc().create_thread(entry as usize, context))
        }
    };

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

pub fn get_thread<F, T>(process: Option<&Process>, id: u64, f: F) -> Result<T, StandardError>
where
    F: FnOnce(&Thread) -> T,
{
    if let Some(process) = process {
        return process.get_thread(id, f);
    }

    get_current_thread(|thread| thread.process().get_thread(id, f))
}

pub fn get_process(id: u64) -> Result<Arc<Process>, StandardError> {
    ProcessManager::get().get_process(id)
}

pub fn wait_thread(process: Option<&Process>, id: u64) -> Result<isize, StandardError> {
    let local_controller = LocalState::get().process_controller().borrow();

    let queue = process
        .unwrap_or(local_controller.current_thread().process())
        .get_thread(id, |thread| thread.exit_queue())?;

    drop(local_controller);

    ProcessManager::get().r#yield(Some(queue));

    Ok(get_current_thread(|thread| thread.queue_data()))
}

pub fn wait_process(id: u64) -> Result<isize, StandardError> {
    get_process(id)?.wait();
    Ok(get_current_thread(|thread| thread.queue_data()))
}

pub fn exit_thread(exit_code: isize) -> ! {
    get_current_thread(|thread| thread.kill(exit_code));
    ProcessManager::get().r#yield(None);
    panic!("Returned from exit thread yield");
}

pub fn exit_process(exit_code: isize) -> ! {
    get_current_thread(|thread| thread.process().kill(exit_code));
    ProcessManager::get().r#yield(None);
    panic!("Return from exit process yield");
}
