use super::{set_gs, LocalState};
use core::{cell::RefCell, ptr::null};

pub struct LocalStateContainer<'local> {
    local_state: LocalState<'local>,
    local_state_ref: RefCell<*const LocalState<'local>>,
}

impl<'local> LocalStateContainer<'local> {
    pub(super) fn new(local_state: LocalState<'local>) -> Self {
        LocalStateContainer {
            local_state,
            local_state_ref: RefCell::new(null()),
        }
    }

    pub fn set_active(&'local self) -> &'local LocalState {
        *self.local_state_ref.borrow_mut() = &self.local_state;

        let ptr = self.local_state_ref.as_ptr();
        unsafe { set_gs(ptr as usize) };
        LocalState::get()
    }
}
