use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name, Interpreter, Result},
    namespace::objects::Processor,
    parser,
};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, processor: parser::Processor) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Processor ({}, {}, {}, {})",
        processor.name(),
        processor.id(),
        processor.address(),
        processor.length()
    );

    let parent = get_parent!(interpreter, processor.name())?;
    let processor_object = Processor::new(
        Some(&parent),
        unwrap_object_name!(processor.name())?,
        processor.id(),
        processor.address(),
        processor.length(),
    );
    add_child!(parent, processor_object, processor.name())
}
