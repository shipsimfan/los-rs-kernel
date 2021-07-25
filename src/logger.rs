use crate::{
    device::{get_device, DeviceBox},
    locks::Mutex,
};
use core::fmt::{self, Write};

struct BootVideoLogger(DeviceBox);

static BOOT_VIDEO_LOGGER: Mutex<Option<BootVideoLogger>> = Mutex::new(None);

pub fn enable_boot_video_logging() {
    let boot_video_device = match get_device("/boot_video") {
        Ok(device) => device,
        Err(status) => panic!("Unable to get boot video device: {}", status),
    };

    (*BOOT_VIDEO_LOGGER.lock()) = Some(BootVideoLogger(boot_video_device));
}

impl Write for BootVideoLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.0.lock().write(0, s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[doc(hidden)]
pub fn _log(args: fmt::Arguments) {
    let mut lock = BOOT_VIDEO_LOGGER.lock();
    match &mut *lock {
        None => {}
        Some(logger) => logger.write_fmt(args).unwrap(),
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::logger::_log(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! logln {
    () => ($crate::log!("\n"));
    ($($arg:tt)*) => ($crate::log!("{}\n", format_args!($($arg)*)));
}
