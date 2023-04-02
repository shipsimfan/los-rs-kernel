use super::{data_object, DataObject, Interpreter, Result};
use crate::parser::{Argument, NameString};

pub(super) fn execute(
    interpreter: &mut Interpreter,
    argument: &Argument,
    name: &NameString,
) -> Result<DataObject> {
    match argument {
        Argument::DataObject(data_object) => data_object::execute(interpreter, data_object, name),
    }
}
