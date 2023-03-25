use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml) enum ArgObj {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
}

const ARG_0_OP: u8 = 0x68;
const ARG_1_OP: u8 = 0x69;
const ARG_2_OP: u8 = 0x6A;
const ARG_3_OP: u8 = 0x6B;
const ARG_4_OP: u8 = 0x6C;
const ARG_5_OP: u8 = 0x6D;
const ARG_6_OP: u8 = 0x6E;

impl ArgObj {
    pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            ARG_0_OP => {
                stream.next();
                Ok(Some(ArgObj::_0))
            }
            ARG_1_OP => {
                stream.next();
                Ok(Some(ArgObj::_1))
            }
            ARG_2_OP => {
                stream.next();
                Ok(Some(ArgObj::_2))
            }
            ARG_3_OP => {
                stream.next();
                Ok(Some(ArgObj::_3))
            }
            ARG_4_OP => {
                stream.next();
                Ok(Some(ArgObj::_4))
            }
            ARG_5_OP => {
                stream.next();
                Ok(Some(ArgObj::_5))
            }
            ARG_6_OP => {
                stream.next();
                Ok(Some(ArgObj::_6))
            }
            _ => Ok(None),
        }
    }
}

impl Display for ArgObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Arg {}",
            match self {
                ArgObj::_0 => 0,
                ArgObj::_1 => 1,
                ArgObj::_2 => 2,
                ArgObj::_3 => 3,
                ArgObj::_4 => 4,
                ArgObj::_5 => 5,
                ArgObj::_6 => 6,
            }
        )
    }
}

impl_core_display!(ArgObj);
