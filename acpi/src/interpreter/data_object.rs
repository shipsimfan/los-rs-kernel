use super::Interpreter;
use crate::parser;

pub(super) enum DataObject {
    Integer(u64),
    String(parser::String),
}

pub(super) fn execute(_: &mut Interpreter, data_object: &parser::DataObject) -> DataObject {
    match data_object {
        parser::DataObject::One => DataObject::Integer(1),
        parser::DataObject::Byte(byte) => DataObject::Integer(*byte as u64),
        parser::DataObject::Word(word) => DataObject::Integer(*word as u64),
        parser::DataObject::String(string) => DataObject::String(string.clone()),
    }
}
