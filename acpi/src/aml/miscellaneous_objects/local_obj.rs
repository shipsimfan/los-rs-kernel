use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml) enum LocalObj {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

const LOCAL_0_OP: u8 = 0x60;
const LOCAL_1_OP: u8 = 0x61;
const LOCAL_2_OP: u8 = 0x62;
const LOCAL_3_OP: u8 = 0x63;
const LOCAL_4_OP: u8 = 0x64;
const LOCAL_5_OP: u8 = 0x65;
const LOCAL_6_OP: u8 = 0x66;
const LOCAL_7_OP: u8 = 0x67;

impl LocalObj {
    pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            LOCAL_0_OP => {
                stream.next();
                Ok(Some(LocalObj::_0))
            }
            LOCAL_1_OP => {
                stream.next();
                Ok(Some(LocalObj::_1))
            }
            LOCAL_2_OP => {
                stream.next();
                Ok(Some(LocalObj::_2))
            }
            LOCAL_3_OP => {
                stream.next();
                Ok(Some(LocalObj::_3))
            }
            LOCAL_4_OP => {
                stream.next();
                Ok(Some(LocalObj::_4))
            }
            LOCAL_5_OP => {
                stream.next();
                Ok(Some(LocalObj::_5))
            }
            LOCAL_6_OP => {
                stream.next();
                Ok(Some(LocalObj::_6))
            }
            LOCAL_7_OP => {
                stream.next();
                Ok(Some(LocalObj::_7))
            }
            _ => Ok(None),
        }
    }
}

impl Display for LocalObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Local {}",
            match self {
                LocalObj::_0 => 0,
                LocalObj::_1 => 1,
                LocalObj::_2 => 2,
                LocalObj::_3 => 3,
                LocalObj::_4 => 4,
                LocalObj::_5 => 5,
                LocalObj::_6 => 6,
                LocalObj::_7 => 7,
            }
        )
    }
}

impl_core_display!(LocalObj);
