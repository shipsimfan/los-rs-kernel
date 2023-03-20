use super::{ByteStream, Result, AML};

pub(crate) fn parse(definition_block: &[u8]) -> Result<AML> {
    AML::parse(&mut stream)
}
