use crate::LocalState;
use core::ffi::c_void;

pub(super) extern "C" fn thread_enter_kernel(entry: *const c_void, context: usize) -> ! {
    unsafe { post_yield() };

    let function: fn(context: usize) -> isize = unsafe { core::mem::transmute(entry) };

    let result = function(context);

    todo!("Implement exit (Result code: {})", result);
}

pub(super) extern "C" fn thread_enter_user(entry: *const c_void, context: usize) -> ! {
    todo!()
}

unsafe fn post_yield() {
    let local_state = LocalState::get();
    let mut process_controller = local_state.process_controller().borrow_mut();

    process_controller.set_next_thread_and_queue_old();

    local_state
        .critical_state()
        .leave(process_controller.take_key())
}
