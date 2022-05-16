#![no_std]

use alloc::boxed::Box;
use base::{critical::CriticalLock, error::TIME_MODULE_NUMBER, log_info, queue::SortedQueue};
use core::arch::asm;
use process::{Mutex, ProcessTypes, SortedThreadQueue};

extern crate alloc;

#[derive(Debug)]
struct AlreadyRegisteredError;

const MODULE_NAME: &str = "Time";

static mut TIME_INITIALIZED: bool = false;

process::static_generic!(
    process::Mutex<
        Option<
            base::multi_owner::Reference<
                alloc::boxed::Box<dyn device::Device>,
                process::Mutex<alloc::boxed::Box<dyn device::Device>, T>,
            >,
        >,
        T,
    >,
    system_timer
);

static mut SYSTEM_TIME: usize = 0;

// Real-time variables
static mut EPOCH_TIME: isize = 0;
static mut SYSTEM_OFFSET: usize = 0;
static mut TIME_ZONE: isize = 0;

// Alarms
process::static_generic!(process::SortedThreadQueue<usize, T>, sleeping_threads);
process::static_generic!(
    base::critical::CriticalLock<
        base::queue::SortedQueue<usize, base::multi_owner::Reference<process::Process<T>>>,
    >,
    alarms
);

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!TIME_INITIALIZED);
        TIME_INITIALIZED = true;
    }

    system_timer::initialize::<T>(Mutex::new(None));

    sleeping_threads::initialize::<T>(SortedThreadQueue::new());

    alarms::initialize::<T>(CriticalLock::new(SortedQueue::new()));

    log_info!("Initialized!");
}

pub fn register_system_timer<T: ProcessTypes + 'static>(
    timer_path: &str,
) -> base::error::Result<unsafe fn()> {
    let timer = device::get_device::<T>(timer_path)?;

    let mut system_timer = system_timer::get::<T>().lock();
    if system_timer.is_some() {
        Err(Box::new(AlreadyRegisteredError))
    } else {
        log_info!("\"{}\" registered as system timer", timer_path);

        *system_timer = Some(timer);
        Ok(millisecond_tick::<T>)
    }
}

pub fn set_timezone(offset: isize, dst: bool) {
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

pub fn sync_offset() {
    unsafe { SYSTEM_OFFSET = SYSTEM_TIME % 1000 };
}

#[inline(always)]
pub fn current_time_millis() -> usize {
    unsafe { SYSTEM_TIME }
}

pub fn sleep<T: ProcessTypes + 'static>(duration: usize) {
    let start = current_time_millis();
    let end = start + duration;

    if duration < 10 {
        while current_time_millis() < end {
            unsafe { asm!("hlt") };
        }
    } else {
        let sleeping_queue = sleeping_threads::get::<T>().current_queue(end);
        process::yield_thread(Some(sleeping_queue))
    }
}

pub fn set_alarm<T: ProcessTypes + 'static>(time: usize) {
    let start = current_time_millis();
    let end = start + time;

    alarms::get::<T>().lock().insert(
        process::current_thread().lock(|thread| thread.process().as_ref()),
        end,
    )
}

unsafe fn millisecond_tick<T: ProcessTypes + 'static>() {
    SYSTEM_TIME += 1;

    match process::current_thread_option::<T>() {
        Some(thread) => {
            thread.lock(|thread| thread.process().lock(|process| process.increase_time(1)))
        }
        None => {}
    }

    while let Some(thread) = sleeping_threads::get::<T>().pop_le(SYSTEM_TIME) {
        process::queue_thread(thread)
    }

    {
        let mut alarms = alarms::get::<T>().lock();
        while let Some(_process) = alarms.pop_le(SYSTEM_TIME) {
            // TODO: Raise the appropriate signals once they are reintroduced
            // process.raise(SignalType::Alarm as u8)
        }
        drop(alarms)
    }

    if SYSTEM_TIME % 1000 == SYSTEM_OFFSET {
        EPOCH_TIME += 1;
    }

    if SYSTEM_TIME % 10 == 0 {
        base::critical::leave_local_without_sti();
        process::preempt::<T>();
        base::critical::enter_local();
    }
}

impl base::error::Error for AlreadyRegisteredError {
    fn module_number(&self) -> i32 {
        TIME_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::Exists
    }
}

impl core::fmt::Display for AlreadyRegisteredError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "System timer already registered")
    }
}
