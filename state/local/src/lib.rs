#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use core::arch::global_asm;
use critical::CriticalState;
use global_state::GlobalState;
use interrupts::GDT;

global_asm!(include_str!("./gs.asm"));

pub struct LocalState<'a> {
    global_state: Arc<GlobalState>,

    critical_state: CriticalState,

    gdt: &'a GDT<'a>,
}

extern "C" {
    fn get_gs() -> usize;
    fn set_gs(value: usize);
}

pub fn try_get_local<'a>() -> Option<&'a mut LocalState<'a>> {
    let gs = unsafe { get_gs() };
    // TODO: Switch with constant KERNEL_VMA
    if gs <= 0xFFF8000000000000 {
        None
    } else {
        Some(unsafe { &mut *(gs as *mut LocalState) })
    }
}

pub fn get_local<'a>() -> &'a mut LocalState<'a> {
    try_get_local().unwrap()
}

impl<'a> LocalState<'a> {
    pub fn new(gdt: &'a GDT, global: Arc<GlobalState>) -> Self {
        LocalState {
            global_state: global,

            critical_state: CriticalState::new(),

            gdt,
        }
    }

    pub fn gdt(&self) -> &GDT {
        self.gdt
    }
}

impl<'local> critical::LocalState for LocalState<'local> {
    fn try_critical_state<'b>() -> Option<&'b CriticalState> {
        try_get_local().map(|local_state| &local_state.critical_state)
    }
}
