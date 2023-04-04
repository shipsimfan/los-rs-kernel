use super::{Integer, Interpreter, Result};
use crate::parser::{self, NameString};
use alloc::vec::Vec;

pub(crate) enum PackageElement {
    DataObject(DataObject),
    NameString(NameString),
}

pub(crate) enum DataObject {
    Buffer(Vec<u8>),
    Integer(Integer),
    Package(Vec<PackageElement>),
    String(Vec<u8>),
}

pub(super) fn execute(
    interpreter: &mut Interpreter,
    data_object: &parser::DataObject,
    name: &NameString,
) -> Result<DataObject> {
    Ok(match data_object {
        parser::DataObject::Zero => DataObject::Integer(interpreter.create_integer(0)),
        parser::DataObject::One => DataObject::Integer(interpreter.create_integer(1)),
        parser::DataObject::Ones => DataObject::Integer(interpreter.create_integer(0xFF)),
        parser::DataObject::Byte(byte) => {
            DataObject::Integer(interpreter.create_integer(*byte as u64))
        }
        parser::DataObject::Word(word) => {
            DataObject::Integer(interpreter.create_integer(*word as u64))
        }
        parser::DataObject::DWord(d_word) => {
            DataObject::Integer(interpreter.create_integer(*d_word as u64))
        }
        parser::DataObject::QWord(q_word) => {
            DataObject::Integer(interpreter.create_integer(*q_word))
        }

        parser::DataObject::String(string) => DataObject::String(string.to_vec()),
        parser::DataObject::Buffer(buffer) => {
            let buffer_size =
                match super::argument::execute(interpreter, buffer.buffer_size(), name)? {
                    DataObject::Integer(buffer_size) => buffer_size,
                    _ => return Err(super::Error::InvalidType(name.clone())),
                };

            DataObject::Buffer(buffer.to_vec(buffer_size))
        }

        parser::DataObject::Package(package) => {
            let mut elements = Vec::with_capacity(package.elements().len());
            for element in package.elements() {
                elements.push(match element {
                    parser::PackageElement::DataObject(data_object) => {
                        PackageElement::DataObject(execute(interpreter, data_object, name)?)
                    }
                    parser::PackageElement::NameString(name_string) => {
                        PackageElement::NameString(name_string.clone())
                    }
                });
            }
            DataObject::Package(elements)
        }
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
            DataObject::Package(package) => {
                write!(f, "{{ ")?;
                for i in 0..package.len() {
                    match &package[i] {
                        PackageElement::DataObject(data_object) => data_object.fmt(f),
                        PackageElement::NameString(name_string) => name_string.fmt(f),
                    }?;
                    if i < package.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " }}")
            }
            DataObject::String(string) => {
                for byte in string {
                    write!(f, "{}", *byte as char)?;
                }
                Ok(())
            }
        }
    }
}
