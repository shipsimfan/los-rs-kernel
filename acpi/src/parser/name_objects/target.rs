use super::SuperName;
use crate::parser::{next, Result, Stream};

pub(super) enum Target {
    NullName,
    SuperName(SuperName),
}

impl Target {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match next!(stream) {
            0x00 => Ok(Target::NullName),
            _ => {
                stream.prev();
                SuperName::parse(stream).map(|super_name| Target::SuperName(super_name))
            }
        }
    }
}
