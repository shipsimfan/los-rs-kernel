use crate::namespace::{
    display_name, display_prefix, impl_core_display, Children, Display, Node, Scope,
};
use alloc::rc::Rc;
use core::cell::RefCell;

pub(crate) struct ThermalZone {
    scope: Scope,
}

impl ThermalZone {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: [u8; 4],
    ) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(ThermalZone {
            scope: Scope::new_raw(parent, Some(name)),
        }))
    }
}

impl Node for ThermalZone {
    fn name(&self) -> Option<[u8; 4]> {
        self.scope.name()
    }

    fn parent(&self) -> Option<alloc::rc::Rc<core::cell::RefCell<dyn Node>>> {
        self.scope.parent()
    }

    fn as_children(&self) -> Option<&dyn Children> {
        self.scope.as_children()
    }

    fn as_children_mut(&mut self) -> Option<&mut dyn Children> {
        self.scope.as_children_mut()
    }
}

impl Children for ThermalZone {
    fn children(&self) -> &[alloc::rc::Rc<core::cell::RefCell<dyn Node>>] {
        self.scope.children()
    }

    fn get_child(&self, name: [u8; 4]) -> Option<alloc::rc::Rc<core::cell::RefCell<dyn Node>>> {
        self.scope.get_child(name)
    }

    fn add_child(&mut self, child: alloc::rc::Rc<core::cell::RefCell<dyn Node>>) -> bool {
        self.scope.add_child(child)
    }
}

impl Display for ThermalZone {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "ThermalZone (")?;
        display_name!(f, self.name().unwrap());
        write!(f, ") {{")?;

        if self.scope.children().len() == 0 {
            return writeln!(f, " }}");
        }

        writeln!(f)?;

        let children = self.scope.children();
        for i in 0..children.len() {
            children[i]
                .borrow()
                .display(f, depth + 1, i == children.len() - 1)?;
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

impl_core_display!(ThermalZone);
