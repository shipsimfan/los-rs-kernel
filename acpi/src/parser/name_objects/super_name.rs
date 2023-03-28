use super::SimpleName;
use crate::parser::{match_next, next, Result, Stream, DEBUG_OP, EXT_OP_PREFIX};

pub(super) enum SuperName {
    Debug,
    //ReferenceTypeOpcode(ReferenceTypeOpcode),
    SimpleName(SimpleName),
}

impl SuperName {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        /*match ReferenceTypeOpcode::parse(stream)? {
            Some(reference_type_opcode) => {
                return Ok(SuperName::ReferenceTypeOpcode(reference_type_opcode))
            }
            None => {}
        }*/

        match next!(stream) {
            EXT_OP_PREFIX => match_next!(stream,
                DEBUG_OP => Ok(SuperName::Debug)
            ),
            _ => {
                stream.prev();
                SimpleName::parse(stream).map(|simple_name| SuperName::SimpleName(simple_name))
            }
        }
    }
}
