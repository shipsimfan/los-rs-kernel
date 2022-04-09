use crate::{
    critical::CriticalLock,
    device::{self, DeviceReference},
    error,
    ipc::SignalType,
    locks::Mutex,
    process::{self, ProcessReference, SortedThreadQueue},
    queue::SortedQueue,
};
use core::arch::asm;

static SYSTEM_TIMER: Mutex<Option<DeviceReference>> = Mutex::new(None);
static mut SYSTEM_TIME: usize = 0;

static mut EPOCH_TIME: isize = 0;
static mut SYSTEM_OFFSET: usize = 0;
static mut TIME_ZONE: isize = 0;

static SLEEPING_THREADS: SortedThreadQueue<usize> = SortedThreadQueue::new();
static ALARMS: CriticalLock<SortedQueue<usize, ProcessReference>> =
    CriticalLock::new(SortedQueue::new());

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

    match process::get_current_thread_option() {
        Some(thread) => thread.process().unwrap().increase_time(1),
        None => {}
    }

    while let Some(thread) = SLEEPING_THREADS.pop_le(SYSTEM_TIME) {
        process::queue_thread(thread)
    }

    {
        let mut alarms = ALARMS.lock();
        while let Some(process) = alarms.pop_le(SYSTEM_TIME) {
            process.raise(SignalType::Alarm as u8)
        }
        drop(alarms)
    }

    if SYSTEM_TIME % 1000 == SYSTEM_OFFSET {
        EPOCH_TIME += 1;
    }

    if SYSTEM_TIME % 10 == 0 {
        crate::critical::leave_local_without_sti();
        process::preempt();
        crate::critical::enter_local();
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

    if duration < 10 {
        while current_time_millis() < end {
            unsafe { asm!("hlt") };
        }
    } else {
        let sleeping_queue = SLEEPING_THREADS.into_current_queue(end);
        process::yield_thread(Some(sleeping_queue))
    }
}

pub fn set_alarm(time: usize) {
    let start = current_time_millis();
    let end = start + time;

    ALARMS
        .lock()
        .insert(process::get_current_thread().process().unwrap(), end)
}
