use super::{Error, Interpreter, Result};
use crate::parser::{Term, TermList};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, term_list: &mut TermList) -> Result<()> {
    for term in term_list {
        let term = term?;

        match term {
            Term::Method(method) => log_debug!(
                interpreter.logger(),
                "Method ({}, {}, {}, {})",
                method.name(),
                method.arg_count(),
                method.serialized(),
                method.sync_level()
            ),
            Term::Scope(mut scope) => {
                log_debug!(interpreter.logger(), "Scope ({})", scope.name());

                // Change the context
                let new_node = interpreter
                    .get_node(scope.name())
                    .ok_or_else(|| Error::UnknownName(scope.name().clone()))?;
                interpreter.push_current_node(new_node);

                // Read the term list
                execute(interpreter, scope.term_list())?;

                interpreter.pop_current_node();
            }
        }
    }

    Ok(())
}
