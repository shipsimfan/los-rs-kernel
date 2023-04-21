use crate::{ExceptionInfo, ExceptionType, InterruptController, LocalState, ProcessManager};
use core::panic;

struct UnhandledException(u64);

#[no_mangle]
#[allow(unused)]
extern "C" fn exception_handler(info: ExceptionInfo) {
    // Enter critical
    let key = LocalState::try_get()
        .map(|local_state| unsafe { local_state.critical_state().enter_assert() });

    // Get the exception
    let controller = InterruptController::get().lock();
    let handler = controller.exceptions()[info.interrupt()];
    drop(controller);

    // Execute the exception
    match handler {
        Some(handler) => handler(info),
        None => panic!("{}", UnhandledException(info.interrupt())),
    }

    // Leave critical
    key.map(|key| unsafe { LocalState::get().critical_state().leave_without_sti(key) });

    // Check to see if the current thread is killed
    if LocalState::get()
        .process_controller()
        .borrow()
        .current_thread()
        .is_killed()
    {
        ProcessManager::get().r#yield(None);
    }
}

impl core::fmt::Display for UnhandledException {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Unhandled exception \"")?;

        match ExceptionType::parse(self.0) {
            Some(exception_type) => write!(f, "{}", exception_type),
            None => write!(f, "Unknown"),
        }?;

        write!(f, "\" ({})", self.0)
    }
}
