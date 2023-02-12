use crate::critical::CriticalState;
use crate::gdt::GDT;
use core::arch::global_asm;

mod container;

pub use container::LocalStateContainer;

pub struct LocalState<'local> {
    critical_state: CriticalState,

    gdt: &'local GDT<'local>,
}

global_asm!(include_str!("./gs.asm"));

extern "C" {
    fn get_gs() -> usize;
    fn set_gs(local_state: usize);
}

impl<'local> LocalState<'local> {
    pub fn new(gdt: &'local GDT) -> LocalStateContainer<'local> {
        LocalStateContainer::new(LocalState {
            critical_state: CriticalState::new(),

            gdt,
        })
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
