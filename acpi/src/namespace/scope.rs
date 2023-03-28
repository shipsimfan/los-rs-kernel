use super::{display_prefix, impl_core_display, Children, Display, Node};
use crate::namespace::display_name;
use alloc::{
    rc::{Rc, Weak},
    vec::Vec,
};
use core::cell::RefCell;

pub(super) struct Scope {
    parent: Option<Weak<RefCell<dyn Node>>>,
    name: Option<[u8; 4]>,
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl Scope {
    pub(super) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: Option<[u8; 4]>,
    ) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(Scope {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            children: Vec::new(),
        }))
    }
}

impl Node for Scope {
    fn parent(&self) -> Option<Rc<RefCell<dyn Node>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    fn name(&self) -> Option<[u8; 4]> {
        self.name
    }
}

impl Children for Scope {
    fn children(&self) -> &[Rc<RefCell<dyn Node>>] {
        &self.children
    }

    fn add_child(&mut self, new_child: Rc<RefCell<dyn Node>>) -> bool {
        for child in &self.children {
            if child.borrow().name() == new_child.borrow().name() {
                return false;
            }
        }

        self.children.push(new_child);
        true
    }
}

impl Display for Scope {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Scope (")?;

        match self.name {
            Some(name) => display_name!(f, name),
            None => write!(f, "\\")?,
        }

        if self.children.len() == 0 {
            return writeln!(f, ") {{ }}");
        }

        writeln!(f, ") {{")?;

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

impl_core_display!(Scope);
