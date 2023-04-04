macro_rules! add_child {
    ($parent: expr, $child_object: expr, $name: expr) => {{
        let mut parent = $parent.borrow_mut();
        let parent = parent
            .as_children_mut()
            .ok_or_else(|| Error::InvalidParent($name.clone()))?;

        if parent.add_child($child_object) {
            Ok(())
        } else {
            Err(Error::NameCollision($name.clone()))
        }
    }};
}

macro_rules! downcast_node {
    ($node: expr, $ty: ty, $name: expr) => {
        $node
            .as_any_mut()
            .downcast_mut::<$ty>()
            .ok_or_else(|| Error::InvalidParent($name.clone()))
    };
}

macro_rules! get_node {
    ($interpreter: expr, $name: expr) => {
        $interpreter
            .get_node($name, true)
            .ok_or_else(|| Error::UnknownName($name.clone()))
    };
}

macro_rules! get_parent {
    ($interpreter: expr, $name: expr) => {
        $interpreter
            .get_node($name, false)
            .ok_or_else(|| Error::UnknownName($name.clone()))
    };
}

macro_rules! unwrap_object_name {
    ($name: expr) => {
        $name
            .name()
            .ok_or_else(|| Error::InvalidName($name.clone()))
    };
}

macro_rules! unwrap_type {
    ($expr: expr, $ty: ident, $name: expr) => {
        match $expr {
            $crate::interpreter::data_object::DataObject::$ty(value) => Ok(value),
            _ => Err(Error::InvalidType($name.clone())),
        }
    };
}

pub(super) use {add_child, downcast_node, get_node, get_parent, unwrap_object_name, unwrap_type};
