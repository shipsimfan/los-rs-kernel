use base::log_debug;

use super::{Interpreter, Result};
use crate::parser::{Term, TermList};

pub(super) fn execute(interpreter: &mut Interpreter, term_list: &mut TermList) -> Result<()> {
    for term in term_list {
        let term = term?;

        match term {
            Term::Scope(scope) => log_debug!(interpreter.logger(), "Scope ({})", scope.name()),
        }
    }

    Ok(())
}
