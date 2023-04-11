use crate::parser::{next, Result, Stream};

pub(crate) struct ArgObj(u8);

impl ArgObj {
    pub(in crate::parser) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        let c = next!(stream, "Arg Obj");

        if c >= 0x68 && c <= 0x6E {
            Ok(Some(ArgObj(c - 0x68)))
        } else {
            stream.prev();
            Ok(None)
        }
    }
}

impl core::fmt::Display for ArgObj {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Argument {}", self.0)
    }
}
