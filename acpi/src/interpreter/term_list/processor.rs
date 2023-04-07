use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name, Interpreter, Result},
    namespace::objects::Processor,
    parser,
};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    mut processor: parser::Processor<'a>,
) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Processor ({}, {}, {}, {})",
        processor.name(),
        processor.id(),
        processor.address(),
        processor.length()
    );

    if interpreter.executing_method() {
        todo!();
    }

    let parent = get_parent!(interpreter, processor.name())?;
    let processor_object = Processor::new(
        Some(&parent),
        unwrap_object_name!(processor.name())?,
        processor.id(),
        processor.address(),
        processor.length(),
    );
    add_child!(parent, processor_object.clone(), processor.name())?;

    interpreter.push_current_node(processor_object);
    super::execute(interpreter, processor.term_list())?;
    interpreter.pop_current_node();

    Ok(())
}
