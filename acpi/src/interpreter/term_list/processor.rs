use base::log_debug;

use crate::{
    interpreter::{Interpreter, Result},
    parser,
};

pub(super) fn execute(interpreter: &mut Interpreter, processor: parser::Processor) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Processor ({}, {}, {}, {})",
        processor.name(),
        processor.id(),
        processor.address(),
        processor.length()
    );

    Ok(())
}
