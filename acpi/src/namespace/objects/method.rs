use crate::{
    namespace::{display_name, display_prefix, impl_core_display, Display, Node},
    parser::TermList,
};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Method<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: [u8; 4],
    arg_count: u8,
    serialized: bool,
    sync_level: u8,
    term_list: TermList<'a>,
    wide_integers: bool,
}

impl<'a> Method<'a> {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<Node<'a>>>>,
        name: [u8; 4],
        arg_count: u8,
        serialized: bool,
        sync_level: u8,
        term_list: TermList<'a>,
        wide_integers: bool,
    ) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::Method(Method {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            arg_count,
            serialized,
            sync_level,
            term_list,
            wide_integers
        })))
    }

    pub(in crate::namespace) fn name(&self) -> [u8; 4] {
        self.name
    }

    pub(in crate::namespace) fn parent(&self) -> Option<alloc::rc::Rc<RefCell<Node<'a>>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    pub(crate) fn argument_count(&self) -> u8 {
        self.arg_count
    }

    pub(crate) fn term_list(&self) -> TermList<'a> {
        self.term_list.clone()
    }

    pub(crate) fn wide_integers(&self) -> bool {
        self.wide_integers
    }
}

impl<'a> Display for Method<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Method (")?;
        display_name!(f, self.name);
        writeln!(
            f,
            ", {}, {}, {})",
            self.arg_count, self.serialized, self.sync_level
        )
    }
}

impl_core_display!(Method);
