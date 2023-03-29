use super::{Interpreter, Result};
use crate::parser::Field;
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, field: Field) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Field ({}, {}) {{ {} bytes... }}",
        field.name(),
        field.flags(),
        field.field_units().len(),
    );

    Ok(())
}
