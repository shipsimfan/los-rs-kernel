#![no_std]

extern crate alloc;

use core::arch::global_asm;

use alloc::sync::Arc;
use critical::CriticalState;
use global_state::GlobalState;

global_asm!(include_str!("./gs.asm"));

pub struct LocalState {
    critical_state: CriticalState,
    global_state: Arc<GlobalState>,
}

extern "C" {
    fn get_gs() -> usize;
    fn set_gs(value: usize);
}

pub fn try_get_local<'a>() -> Option<&'a mut LocalState> {
    let gs = unsafe { get_gs() };
    // TODO: Switch with constant KERNEL_VMA
    if gs <= 0xFFF8000000000000 {
        None
    } else {
        Some(unsafe { &mut *(gs as *mut LocalState) })
    }
}

pub fn get_local<'a>() -> &'a mut LocalState {
    try_get_local().unwrap()
}

impl LocalState {
    pub fn new<'a>(global: Arc<GlobalState>) -> Self {
        LocalState {
            critical_state: CriticalState::new(),
            global_state: global,
        }
    }
}

impl critical::LocalState for LocalState {
    fn try_critical_state<'a>() -> Option<&'a CriticalState> {
        try_get_local().map(|local_state| &local_state.critical_state)
    }
}
