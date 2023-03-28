use super::{Error, Interpreter, Result};
use crate::parser::Scope;
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, mut scope: Scope) -> Result<()> {
    log_debug!(interpreter.logger(), "Scope ({})", scope.name());

    // Change the context
    let new_node = interpreter
        .get_node(scope.name())
        .ok_or_else(|| Error::UnknownName(scope.name().clone()))?;
    interpreter.push_current_node(new_node);

    // Read the term list
    super::execute(interpreter, scope.term_list())?;

    interpreter.pop_current_node();

    Ok(())
}
