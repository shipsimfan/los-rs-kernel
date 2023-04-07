use super::{DataObject, Interpreter, Result};
use crate::parser::{Term, TermList};

mod device;
mod field;
mod method;
mod mutex;
mod name;
mod op_region;
mod power_resource;
mod processor;
mod scope;
mod thermal_zone;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    term_list: &mut TermList<'a>,
) -> Result<Option<DataObject>> {
    for term in term_list {
        let term = term?;

        match term {
            Term::Device(device) => device::execute(interpreter, device),
            Term::Field(field) => field::execute(interpreter, field),
            Term::Method(method) => method::execute(interpreter, method),
            Term::Mutex(mutex) => mutex::execute(interpreter, mutex),
            Term::Name(name) => name::execute(interpreter, name),
            Term::OpRegion(op_region) => op_region::execute(interpreter, op_region),
            Term::PowerResource(power_resource) => {
                power_resource::execute(interpreter, power_resource)
            }
            Term::Processor(processor) => processor::execute(interpreter, processor),
            Term::Scope(scope) => scope::execute(interpreter, scope),
            Term::ThermalZone(thermal_zone) => thermal_zone::execute(interpreter, thermal_zone),
        }?
    }

    Ok(None)
}
