use crate::aml::{next, ByteStream, Result};

pub(in crate::aml) struct NameSeg {
    name: [u8; 4],
}

impl NameSeg {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let c = next!(stream);
        if !c.is_ascii_uppercase() && c != b'_' {
            return Err(crate::aml::Error::UnexpectedByte(c));
        }

        let mut name = [c, 0, 0, 0];
        for i in 1..4 {
            let c = next!(stream);
            if !c.is_ascii_uppercase() && !c.is_ascii_digit() && c != b'_' {
                return Err(crate::aml::Error::UnexpectedByte(c));
            }
            name[i] = c;
        }

        Ok(NameSeg { name })
    }
}

impl core::fmt::Display for NameSeg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.name[0] as char, self.name[1] as char, self.name[2] as char, self.name[3] as char
        )
    }
}
