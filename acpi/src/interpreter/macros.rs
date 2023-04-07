macro_rules! add_child {
    ($parent: expr, $child_object: expr, $name: expr) => {{
        let mut parent = $parent.borrow_mut();
        let parent = parent
            .children_mut()
            .ok_or_else(|| $crate::interpreter::Error::InvalidParent($name.clone()))?;

        if parent.add_child($child_object) {
            Ok(())
        } else {
            Err($crate::interpreter::Error::NameCollision($name.clone()))
        }
    }};
}

macro_rules! downcast_node {
    ($node: expr, $ty: ident, $name: expr) => {
        match &mut *$node {
            $crate::namespace::Node::$ty(value) => Ok(value),
            _ => Err($crate::interpreter::Error::InvalidParent($name.clone())),
        }
    };
}

macro_rules! get_node {
    ($interpreter: expr, $name: expr) => {
        $interpreter
            .get_node($name, true)
            .ok_or_else(|| $crate::interpreter::Error::UnknownName($name.clone()))
    };
}

macro_rules! get_parent {
    ($interpreter: expr, $name: expr) => {
        $interpreter
            .get_node($name, false)
            .ok_or_else(|| $crate::interpreter::Error::UnknownName($name.clone()))
    };
}

macro_rules! unwrap_object_name {
    ($name: expr) => {
        $name
            .name()
            .ok_or_else(|| $crate::interpreter::Error::InvalidName($name.clone()))
    };
}

macro_rules! unwrap_type {
    ($expr: expr, $ty: ident, $name: expr) => {
        match $expr {
            $crate::interpreter::data_object::DataObject::$ty(value) => Ok(value),
            _ => Err($crate::interpreter::Error::InvalidType($name.clone())),
        }
    };
}

pub(super) use {add_child, downcast_node, get_node, get_parent, unwrap_object_name, unwrap_type};
