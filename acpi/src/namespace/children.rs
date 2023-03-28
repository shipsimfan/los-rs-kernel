use super::Node;
use alloc::rc::Rc;
use core::cell::RefCell;

pub(crate) trait Children: Node {
    fn children(&self) -> &[Rc<RefCell<dyn Node>>];

    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>) -> bool;

    fn get_child(&self, name: [u8; 4]) -> Option<Rc<RefCell<dyn Node>>> {
        let mut result = None;
        for child in self.children() {
            if let Some(child_name) = child.borrow().name() {
                if child_name == name {
                    result = Some(child.clone());
                    break;
                }
            }
        }

        result
    }
}
