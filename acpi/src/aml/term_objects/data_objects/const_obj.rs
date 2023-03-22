use crate::aml::{impl_core_display, match_next, Display, Result, Stream};

enum ConstObjClass {
    One,
    Zero,
}

pub(in crate::aml::term_objects) struct ConstObj {
    offset: usize,

    class: ConstObjClass,
}

const ZERO_OP: u8 = 0x00;
const ONE_OP: u8 = 0x01;

impl ConstObj {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset();

        match_next!(stream,
            ZERO_OP => Ok(ConstObj { class: ConstObjClass::Zero, offset })
            ONE_OP => Ok(ConstObj {
                class: ConstObjClass::One,
                offset
            })
        )
    }
}

impl Display for ConstObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Const Object @ {}: {}", self.offset, self.class)
    }
}

impl_core_display!(ConstObj);

impl core::fmt::Display for ConstObjClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConstObjClass::One => 1,
                ConstObjClass::Zero => 0,
            }
        )
    }
}
