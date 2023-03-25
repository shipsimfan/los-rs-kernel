use super::{
    impl_core_display, peek, peek_ahead, term_objects::ReferenceTypeOpcode, Debug, Display, Result,
    SimpleName, Stream,
};

pub(super) enum SuperName {
    Debug(Debug),
    ReferenceTypeOpcode(ReferenceTypeOpcode),
    SimpleName(SimpleName),
}

const EXT_OP_PREFIX: u8 = 0x5B;

const DEBUG_OP: u8 = 0x31;

impl SuperName {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match ReferenceTypeOpcode::parse(stream)? {
            Some(reference_type_opcode) => {
                return Ok(SuperName::ReferenceTypeOpcode(reference_type_opcode))
            }
            None => {}
        }

        match peek!(stream) {
            EXT_OP_PREFIX => match peek_ahead!(stream) {
                DEBUG_OP => {
                    stream.next();
                    stream.next();
                    return Debug::parse(stream).map(|debug| SuperName::Debug(debug));
                }
                _ => {}
            },
            _ => {}
        }

        SimpleName::parse(stream).map(|simple_name| SuperName::SimpleName(simple_name))
    }
}

impl Display for SuperName {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            SuperName::Debug(debug) => debug.display(f, depth, last),
            SuperName::ReferenceTypeOpcode(reference_type_opcode) => {
                reference_type_opcode.display(f, depth, last)
            }
            SuperName::SimpleName(simple_name) => simple_name.display(f, depth, last),
        }
    }
}

impl_core_display!(SuperName);
