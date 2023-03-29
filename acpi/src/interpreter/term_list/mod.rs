use super::{Error, Interpreter, Result};
use crate::parser::{Term, TermList};

mod field;
mod method;
mod op_region;
mod scope;

pub(super) fn execute(interpreter: &mut Interpreter, term_list: &mut TermList) -> Result<()> {
    for term in term_list {
        let term = term?;

        match term {
            Term::Field(field) => field::execute(interpreter, field),
            Term::Method(method) => method::execute(interpreter, method),
            Term::OpRegion(op_region) => op_region::execute(interpreter, op_region),
            Term::Scope(scope) => scope::execute(interpreter, scope),
        }?
    }

    Ok(())
}
