use super::{DefField, DefOpRegion};
use crate::aml::{next, ASTNode, ByteStream, Result};

pub(in crate::aml) enum NamedObject {
    DefField(DefField),
    DefOpRegion(DefOpRegion),
}

const EXT_OP_PREFIX: u8 = 0x5B;

const FIELD_OP: u8 = 0x81;
const OP_REGION_OP: u8 = 0x80;

impl NamedObject {
    pub(in crate::aml) fn parse(stream: &mut ByteStream, c: u8) -> Result<Option<Self>> {
        match c {
            EXT_OP_PREFIX => {
                let c = next!(stream);
                match c {
                    FIELD_OP => Ok(Some(NamedObject::DefField(DefField::parse(stream)?))),
                    OP_REGION_OP => Ok(Some(NamedObject::DefOpRegion(DefOpRegion::parse(stream)?))),
                    _ => Err(crate::aml::Error::UnexpectedByte(c)),
                }
            }
            _ => Ok(None),
        }
    }
}

impl ASTNode for NamedObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            NamedObject::DefOpRegion(op_region) => op_region.display(f, depth),
            NamedObject::DefField(field) => field.display(f, depth),
        }
    }
}
