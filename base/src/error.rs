use core::fmt::{Debug, Display};

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
