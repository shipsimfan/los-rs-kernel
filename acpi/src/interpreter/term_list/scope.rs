use super::{Interpreter, Result};
use crate::{interpreter::get_node, parser::Scope};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    mut scope: Scope<'a>,
) -> Result<()> {
    log_debug!(interpreter.logger(), "Scope ({})", scope.name());

    if interpreter.executing_method() {
        todo!();
    }

    // Change the context
    let new_node = get_node!(interpreter, scope.name())?;

    interpreter.push_current_node(new_node);

    // Read the term list
    super::execute(interpreter, scope.term_list())?;

    interpreter.pop_current_node();

    Ok(())
}
