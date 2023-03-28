use super::Node;
use alloc::rc::Rc;
use core::cell::RefCell;

pub(super) trait Children: Node {
    fn children(&self) -> &[Rc<RefCell<dyn Node>>];

    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>) -> bool;
}
