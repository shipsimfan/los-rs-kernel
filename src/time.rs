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

        /*if SYSTEM_TIME % 10 == 0 {
            crate::process::preempt();
        }*/
    }
}
