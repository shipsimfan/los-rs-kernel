use crate::{
    device::{self, DeviceBox},
    error,
    locks::Mutex,
    process,
};

static SYSTEM_TIMER: Mutex<Option<DeviceBox>> = Mutex::new(None);
static mut SYSTEM_TIME: usize = 0;

static mut EPOCH_TIME: isize = 0;
static mut SYSTEM_OFFSET: usize = 0;
static mut TIME_ZONE: isize = 0;

pub fn register_system_timer(timer_path: &str) -> error::Result<()> {
    let timer = device::get_device(timer_path)?;

    let mut system_timer = SYSTEM_TIMER.lock();
    if system_timer.is_some() {
        Err(error::Status::Exists)
    } else {
        *system_timer = Some(timer);
        Ok(())
    }
}

pub fn _set_timezone(offset: isize, dst: bool) {
    unsafe { TIME_ZONE = (offset & !1) | if dst { 1 } else { 0 } };
}

pub fn get_timezone() -> isize {
    unsafe { TIME_ZONE }
}

pub fn set_epoch_time(time: isize) {
    unsafe { EPOCH_TIME = time };
}

pub fn get_epoch_time() -> isize {
    unsafe { EPOCH_TIME }
}

pub unsafe fn millisecond_tick() {
    SYSTEM_TIME += 1;

    match process::get_current_thread_mut_option_cli() {
        Some(thread) => thread.get_process_mut().increase_time(1000),
        None => {}
    }

    if SYSTEM_TIME % 1000 == SYSTEM_OFFSET {
        EPOCH_TIME += 1;
    }

    if SYSTEM_TIME % 10 == 0 {
        crate::process::preempt();
    }
}

pub fn sync_offset() {
    unsafe { SYSTEM_OFFSET = SYSTEM_TIME % 1000 };
}

#[inline(always)]
pub fn current_time_millis() -> usize {
    unsafe { SYSTEM_TIME }
}

pub fn sleep(duration: usize) {
    let start = current_time_millis();
    let end = start + duration;

    while current_time_millis() < end {
        unsafe { asm!("hlt") };
    }
}
