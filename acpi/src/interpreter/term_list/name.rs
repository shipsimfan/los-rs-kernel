use super::{Interpreter, Result};
use crate::parser;
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, name: parser::Name) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Name ({}, {})",
        name.name(),
        name.data_object()
    );

    Ok(())
}
