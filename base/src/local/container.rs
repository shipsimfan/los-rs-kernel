use super::{set_gs, LocalState};
use core::ptr::null;

pub struct LocalStateContainer<'local> {
    local_state: LocalState<'local>,
    gs_ptr: *const LocalState<'local>,
}

impl<'local> LocalStateContainer<'local> {
    pub(super) fn new(local_state: LocalState<'local>) -> Self {
        LocalStateContainer {
            local_state,
            gs_ptr: null(),
        }
    }

    pub fn set_active(&'local mut self) -> &'local LocalState {
        self.gs_ptr = &self.local_state;
        unsafe { set_gs(&self.gs_ptr as *const _ as usize) };
        LocalState::get()
    }
}
