use super::{data_object, DataObject, Interpreter, Result};
use crate::parser::Argument;

pub(super) fn execute(interpreter: &mut Interpreter, argument: &Argument) -> Result<DataObject> {
    match argument {
        Argument::DataObject(data_object) => data_object::execute(interpreter, data_object),
    }
}
