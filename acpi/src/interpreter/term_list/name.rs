use super::{Interpreter, Result};
use crate::{interpreter::Error, namespace::objects::Name, parser};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, name: parser::Name) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Name ({}, {})",
        name.name(),
        name.data_object()
    );

    let parent_rc = interpreter
        .get_node(name.name(), false)
        .ok_or_else(|| Error::UnknownName(name.name().clone()))?;

    let mut parent_ref = parent_rc.borrow_mut();
    let parent = parent_ref
        .as_children_mut()
        .ok_or_else(|| Error::InvalidParent(name.name().clone()))?;

    let data_object =
        super::super::data_object::execute(interpreter, name.data_object(), name.name())?;

    parent.add_child(Name::new(
        Some(&parent_rc),
        name.name()
            .name()
            .ok_or_else(|| Error::InvalidName(name.name().clone()))?,
        data_object,
    ));

    drop(parent_ref);
    interpreter.display_namespace();

    Ok(())
}
