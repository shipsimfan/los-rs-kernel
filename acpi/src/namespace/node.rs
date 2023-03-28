use super::Display;
use alloc::rc::Rc;
use core::{any::Any, cell::RefCell};

pub(crate) trait Node: AsAny + Display {
    fn parent(&self) -> Option<Rc<RefCell<dyn Node>>>;
    fn name(&self) -> Option<[u8; 4]>;
}

pub(crate) trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
