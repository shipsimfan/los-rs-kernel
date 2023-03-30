use super::{Interpreter, Result};
use crate::{
    interpreter::Error,
    namespace::objects::{Field, OperationRegion},
    parser,
};
use alloc::borrow::ToOwned;
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, field: parser::Field) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Field ({}, {})",
        field.name(),
        field.flags(),
    );

    let node = interpreter
        .get_node(field.name(), true)
        .ok_or_else(|| Error::UnknownName(field.name().clone()))?;

    let mut node_ref = node.borrow_mut();
    let node = node_ref
        .as_any_mut()
        .downcast_mut::<OperationRegion>()
        .ok_or_else(|| Error::InvalidParent(field.name().clone()))?;

    node.add_field(Field::new(field.flags(), field.field_units().to_owned()));

    Ok(())
}
