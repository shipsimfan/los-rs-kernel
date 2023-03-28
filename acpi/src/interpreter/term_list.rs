use super::{Interpreter, Result};
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
            Term::Scope(scope) => log_debug!(interpreter.logger(), "Scope ({})", scope.name()),
        }
    }

    Ok(())
}
