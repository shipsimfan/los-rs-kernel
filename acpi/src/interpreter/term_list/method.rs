use super::{Interpreter, Result};
use crate::parser::Method;
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, method: Method) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Method ({}, {}, {}, {})",
        method.name(),
        method.arg_count(),
        method.serialized(),
        method.sync_level()
    );

    Ok(())
}
