use super::{Interpreter, Result};
use crate::{
    interpreter::{downcast_node, get_node},
    namespace::objects::Field,
    parser,
};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    field: parser::Field<'a>,
) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Field ({}, {})",
        field.name(),
        field.flags(),
    );

    if interpreter.executing_method() {
        todo!();
    }

    let node = get_node!(interpreter, field.name())?;

    let mut node = node.borrow_mut();
    let node = downcast_node!(node, OperationRegion, field.name())?;

    node.add_field(Field::new(field.flags(), field.field_units()));

    Ok(())
}
