use base::log_info;

use crate::Registers;

pub type Handler = fn(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    registers: Registers,
    rflags: u64,
    rip: u64,
    rsp: u64,
) -> base::error::Result<isize>;

static mut SYSTEM_CALLS_INITIALIZED: bool = false;

// System call handler does not require a critical lock because it is set once a boot
static mut SYSTEM_CALL_HANDLER: Handler = default_system_call_handler;

extern "C" {
    fn init_system_calls();
}

#[no_mangle]
extern "C" fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    registers: Registers,
    rflags: u64,
    rip: u64,
    rsp: u64,
) -> isize {
    match unsafe {
        SYSTEM_CALL_HANDLER(
            code, arg1, arg2, arg3, arg4, arg5, registers, rflags, rip, rsp,
        )
    } {
        Ok(ret) => ret,
        Err(error) => error.to_status_code(),
    }
}

fn default_system_call_handler(
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: Registers,
    _: u64,
    _: u64,
    _: u64,
) -> base::error::Result<isize> {
    panic!("No system call handler setup!");
}

pub fn initialize(system_call_handler: Handler) {
    log_info!("Initializing system calls . . . ");

    unsafe {
        assert!(!SYSTEM_CALLS_INITIALIZED);
        SYSTEM_CALLS_INITIALIZED = true;

        SYSTEM_CALL_HANDLER = system_call_handler;
    }

    unsafe { init_system_calls() };

    log_info!("Initialized system calls!");
}
