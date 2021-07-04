use crate::{
    device::{self, DeviceBox},
    error::{self, Status},
    locks::Mutex,
};

static SYSTEM_TIMER: Mutex<Option<DeviceBox>> = Mutex::new(None);
static mut SYSTEM_TIME: usize = 0;

pub fn register_system_timer(timer_path: &str) -> error::Result {
    let timer = device::get_device(timer_path)?;

    let mut system_timer = SYSTEM_TIMER.lock();
    if system_timer.is_some() {
        Err(Status::AlreadyExists)
    } else {
        *system_timer = Some(timer);
        Ok(())
    }
}

pub fn millisecond_tick() {
    unsafe {
        SYSTEM_TIME += 1;

        /*
        // Testing Code - displays a message once a second
        if SYSTEM_TIME % 1000 == 0 {
            logln!("System Time: {}s", SYSTEM_TIME / 1000);
        }*/

        if SYSTEM_TIME % 10 == 0 {
            crate::process::preempt();
        }
    }
}

#[inline(always)]
pub fn current_time_millis() -> usize {
    unsafe { SYSTEM_TIME }
}

pub fn sleep(duration: usize) {
    let start = current_time_millis();
    let end = start + duration;

    while current_time_millis() < end {
        unsafe { asm!("sti;hlt") };
    }
}
