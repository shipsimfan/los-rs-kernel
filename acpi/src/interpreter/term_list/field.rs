use super::{Interpreter, Result};
use crate::{
    interpreter::{downcast_node, get_node, Error},
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

    let node = get_node!(interpreter, field.name())?;

    let mut node = node.borrow_mut();
    let node = downcast_node!(node, OperationRegion, field.name())?;

    node.add_field(Field::new(field.flags(), field.field_units().to_owned()));

    Ok(())
}
