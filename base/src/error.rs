use alloc::boxed::Box;
use core::fmt::{Debug, Display};

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub const DEVICE_MODULE_NUMBER: i32 = 0;

pub const UEFI_DRIVER_MODULE_NUMBER: i32 = 0x1000000;

pub trait Error: Debug + Display {
    fn module_number(&self) -> i32;
    fn error_number(&self) -> u32;

    #[inline(always)]
    fn to_status_code(&self) -> isize {
        let module_number = self.module_number();
        assert!(module_number >= 0);

        -((((module_number as usize) << ((core::mem::size_of::<usize>() / 2) * 8))
            + self.error_number() as usize) as isize)
    }
}
