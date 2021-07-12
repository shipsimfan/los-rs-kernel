use crate::logln;

#[no_mangle]
extern "C" fn get_kernel_stack_pointer_and_save_user_stack_pointer(
    user_stack_pointer: usize,
) -> usize {
    let current_thread = crate::process::get_current_thread_mut();
    current_thread.set_user_stack_pointer(user_stack_pointer);
    current_thread.get_kernel_stack_top()
}

#[no_mangle]
extern "C" fn get_user_stack_pointer() -> usize {
    crate::process::get_current_thread().get_user_stack_pointer()
}

#[no_mangle]
extern "C" fn system_call(code: usize, arg1: usize) -> usize {
    match code {
        0 => crate::process::exit_thread(arg1),
        _ => logln!("Invalid system call: {}", code),
    }

    0
}
