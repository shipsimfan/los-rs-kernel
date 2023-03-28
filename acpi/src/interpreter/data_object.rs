use super::{Interpreter, Result};
use crate::parser;

pub(super) enum DataObject {
    Integer(u64),
}

pub(super) fn execute(
    interpreter: &mut Interpreter,
    data_object: &parser::DataObject,
) -> Result<DataObject> {
    match data_object {
        parser::DataObject::One => Ok(DataObject::Integer(1)),
        parser::DataObject::Word(word) => Ok(DataObject::Integer(*word as u64)),
    }
}
