use super::{Interpreter, Result};
use crate::parser::{self, NameString};
use alloc::vec::Vec;

pub(crate) enum DataObject {
    Buffer(Vec<u8>),
    Integer(u64),
    String(Vec<u8>),
}

pub(super) fn execute(
    interpreter: &mut Interpreter,
    data_object: &parser::DataObject,
    name: &NameString,
) -> Result<DataObject> {
    Ok(match data_object {
        parser::DataObject::One => DataObject::Integer(1),
        parser::DataObject::Buffer(buffer) => {
            let buffer_size =
                match super::argument::execute(interpreter, buffer.buffer_size(), name)? {
                    DataObject::Integer(buffer_size) => buffer_size,
                    _ => return Err(super::Error::InvalidType(name.clone())),
                };

            DataObject::Buffer(buffer.to_vec(buffer_size))
        }
        parser::DataObject::Byte(byte) => DataObject::Integer(*byte as u64),
        parser::DataObject::Word(word) => DataObject::Integer(*word as u64),
        parser::DataObject::String(string) => DataObject::String(string.to_vec()),
    })
}

impl core::fmt::Display for DataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DataObject::Buffer(buffer) => {
                write!(f, "[")?;
                for i in 0..buffer.len() {
                    write!(f, "{:#04X}", buffer[i])?;
                    if i < buffer.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            DataObject::Integer(value) => value.fmt(f),
            DataObject::String(string) => {
                for byte in string {
                    write!(f, "{}", *byte as char)?;
                }
                Ok(())
            }
        }
    }
}
