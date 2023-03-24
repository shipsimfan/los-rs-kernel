use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum ConstObj {
    One,
    Ones,
    Zero,
}

const ZERO_OP: u8 = 0x00;
const ONE_OP: u8 = 0x01;
const ONES_OP: u8 = 0xFF;

impl ConstObj {
    pub(super) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            ZERO_OP => {
                stream.next();
                Ok(Some(ConstObj::Zero))
            }
            ONE_OP => {
                stream.next();
                Ok(Some(ConstObj::One))
            }
            ONES_OP => {
                stream.next();
                Ok(Some(ConstObj::Ones))
            }
            _ => Ok(None),
        }
    }
}

impl Display for ConstObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "{:#02X}",
            match self {
                ConstObj::One => 1,
                ConstObj::Ones => 0xFF,
                ConstObj::Zero => 0,
            }
        )
    }
}

impl_core_display!(ConstObj);
