use crate::parser::{next, Result, Stream};

pub(crate) struct LocalObj(u8);

impl LocalObj {
    pub(in crate::parser) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        let c = next!(stream, "Local Obj");

        if c >= 0x60 && c <= 0x67 {
            Ok(Some(LocalObj(c - 0x60)))
        } else {
            stream.prev();
            Ok(None)
        }
    }
}

impl core::fmt::Display for LocalObj {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Local {}", self.0)
    }
}
