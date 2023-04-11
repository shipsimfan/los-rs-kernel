use alloc::vec::Vec;

use crate::{
    parser::{next, Error, Result, Stream},
    String,
};

pub(super) fn parse(stream: &mut Stream) -> Result<String> {
    let mut string = Vec::new();

    let mut c = next!(stream, "String");
    while c != 0x00 {
        if c > 0x7F {
            return Err(Error::unexpected_byte(c, stream.offset() - 1, "String"));
        }

        string.push(c);

        c = next!(stream, "String");
    }

    Ok(unsafe { String::new_unchecked(string) })
}
