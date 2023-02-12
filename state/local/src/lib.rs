#![no_std]
use core::{arch::global_asm, cell::RefCell, ptr::null};
use critical::CriticalState;
use gdt::GDT;

pub mod critical;

pub struct LocalState<'local> {
    critical_state: CriticalState,

    gdt: &'local GDT<'local>,
}

pub struct LocalStateContainer<'local> {
    local_state: LocalState<'local>,
    local_state_ref: RefCell<*const LocalState<'local>>,
}

global_asm!(include_str!("./gs.asm"));

extern "C" {
    fn get_gs() -> usize;
    fn set_gs(local_state: usize);
}

impl<'local> LocalState<'local> {
    pub fn new(gdt: &'local GDT) -> LocalStateContainer<'local> {
        LocalStateContainer {
            local_state: LocalState {
                critical_state: CriticalState::new(),

                gdt,
            },
            local_state_ref: RefCell::new(null()),
        }
    }

    pub fn try_get() -> Option<&'local LocalState<'local>> {
        let gs = unsafe { get_gs() };
        // TODO: Switch with constant KERNEL_VMA
        if gs <= 0xFFFF800000000000 {
            None
        } else {
            Some(unsafe { &*(gs as *const LocalState) })
        }
    }

    pub fn get() -> &'local LocalState<'local> {
        LocalState::try_get().unwrap()
    }

    pub fn critical_state(&self) -> &CriticalState {
        &self.critical_state
    }

    pub fn gdt(&self) -> &GDT {
        self.gdt
    }
}

impl<'local> LocalStateContainer<'local> {
    pub fn set_active(&'local self) -> &'local LocalState {
        *self.local_state_ref.borrow_mut() = &self.local_state;

        let ptr = self.local_state_ref.as_ptr();
        unsafe { set_gs(ptr as usize) };
        LocalState::get()
    }
}
