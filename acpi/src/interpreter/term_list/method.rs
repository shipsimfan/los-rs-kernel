use super::{Interpreter, Result};
use crate::{interpreter::Error, namespace::objects::Method, parser};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, method: parser::Method) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Method ({}, {}, {}, {})",
        method.name(),
        method.arg_count(),
        method.serialized(),
        method.sync_level()
    );

    let parent_rc = interpreter
        .get_node(method.name(), false)
        .ok_or_else(|| Error::UnknownName(method.name().clone()))?;

    let mut parent_ref = parent_rc.borrow_mut();
    let parent = parent_ref
        .as_children_mut()
        .ok_or_else(|| Error::InvalidParent(method.name().clone()))?;

    parent.add_child(Method::new(
        Some(&parent_rc),
        method
            .name()
            .name()
            .ok_or_else(|| Error::InvalidName(method.name().clone()))?,
        method.arg_count(),
        method.serialized(),
        method.sync_level(),
        method.method_size(),
    ));

    Ok(())
}
