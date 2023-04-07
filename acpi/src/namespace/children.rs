use super::{display_prefix, impl_core_display, Display, Node};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

pub(crate) struct Children<'a> {
    children: Vec<Rc<RefCell<Node<'a>>>>,
}

impl<'a> Children<'a> {
    pub(super) fn new() -> Self {
        Children {
            children: Vec::new(),
        }
    }

    pub(crate) fn add_child(&mut self, new_child: Rc<RefCell<Node<'a>>>) -> bool {
        for child in &self.children {
            if child.borrow().name() == new_child.borrow().name() {
                return false;
            }
        }

        self.children.push(new_child);
        true
    }

    pub(crate) fn get_child(&self, name: [u8; 4]) -> Option<Rc<RefCell<Node<'a>>>> {
        let mut result = None;
        for child in &self.children {
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

impl<'a> Display for Children<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        if self.children.len() == 0 {
            return writeln!(f, "{{ }}");
        }

        writeln!(f, "{{")?;

        for i in 0..self.children.len() {
            self.children[i]
                .borrow()
                .display(f, depth + 1, i == self.children.len() - 1)?;
        }

        display_prefix!(f, depth);
        writeln!(f, "}}")?;

        if !last {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(Children);
