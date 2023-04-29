use crate::{
    critical::CriticalState, gdt::GDT, memory::KERNEL_VMA, process::LocalProcessController,
    CriticalRefCell,
};
use alloc::boxed::Box;
use core::{arch::global_asm, ffi::c_void, pin::Pin, ptr::NonNull};

pub struct LocalState {
    critical_state: CriticalState,
    process_controller: CriticalRefCell<LocalProcessController>,

    gdt: Pin<Box<GDT>>,
}

global_asm!(include_str!("./gs.asm"));

extern "C" {
    fn get_gs() -> usize;
    pub(crate) fn set_gs(local_state: usize);
}

impl LocalState {
    pub fn new(null_stack_top: usize, null_gs_ptr: NonNull<*mut c_void>) -> LocalState {
        let gdt = GDT::new();
        gdt.set_active(null_gs_ptr.as_ptr() as usize);

        LocalState {
            critical_state: CriticalState::new(),
            process_controller: CriticalRefCell::new(LocalProcessController::new(null_stack_top)),

            gdt,
        }
    }

    pub fn try_get<'a>() -> Option<&'a LocalState> {
        let gs = unsafe { get_gs() };
        if gs <= KERNEL_VMA {
            None
        } else {
            Some(unsafe { &*(gs as *const _) })
        }
    }

    pub fn get<'a>() -> &'a LocalState {
        LocalState::try_get().unwrap()
    }

    pub fn critical_state(&self) -> &CriticalState {
        &self.critical_state
    }

    pub(crate) fn gdt(&self) -> &GDT {
        &self.gdt
    }

    pub(crate) fn process_controller(&self) -> &CriticalRefCell<LocalProcessController> {
        &self.process_controller
    }
}
