use crate::{critical::CriticalState, gdt::GDT, memory::KERNEL_VMA};
use core::{arch::global_asm, cell::RefCell};

mod container;

pub use container::LocalStateContainer;

pub struct LocalState<'local> {
    critical_state: RefCell<CriticalState>,

    gdt: &'local GDT<'local>,
}

global_asm!(include_str!("./gs.asm"));

extern "C" {
    fn get_gs() -> usize;
    pub(crate) fn set_gs(local_state: usize);
}

impl<'local> LocalState<'local> {
    pub fn new(gdt: &'local GDT<'local>) -> LocalStateContainer<'local> {
        LocalStateContainer::new(LocalState {
            critical_state: RefCell::new(CriticalState::new()),

            gdt,
        })
    }

    pub fn try_get() -> Option<&'local LocalState<'local>> {
        let gs = unsafe { get_gs() };
        if gs <= KERNEL_VMA {
            None
        } else {
            Some(unsafe { &*(gs as *const _) })
        }
    }

    pub fn get() -> &'local LocalState<'local> {
        LocalState::try_get().unwrap()
    }

    pub fn critical_state(&self) -> &RefCell<CriticalState> {
        &self.critical_state
    }

    pub fn gdt(&'local self) -> &'local GDT {
        self.gdt
    }
}
