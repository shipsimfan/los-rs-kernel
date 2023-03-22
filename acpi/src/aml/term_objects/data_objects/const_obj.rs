use crate::aml::{impl_core_display, peek, Display, Result, Stream};

enum ConstObjClass {
    One,
    Ones,
    Zero,
}

pub(in crate::aml::term_objects) struct ConstObj {
    offset: usize,

    class: ConstObjClass,
}

const ZERO_OP: u8 = 0x00;
const ONE_OP: u8 = 0x01;
const ONES_OP: u8 = 0xFF;

impl ConstObj {
    pub(super) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        let offset = stream.offset();

        match peek!(stream) {
            ZERO_OP => {
                stream.next();
                Ok(Some(ConstObj {
                    class: ConstObjClass::Zero,
                    offset,
                }))
            }
            ONE_OP => {
                stream.next();
                Ok(Some(ConstObj {
                    class: ConstObjClass::One,
                    offset,
                }))
            }
            ONES_OP => {
                stream.next();
                Ok(Some(ConstObj {
                    class: ConstObjClass::Ones,
                    offset,
                }))
            }
            _ => Ok(None),
        }
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
            "{:#02X}",
            match self {
                ConstObjClass::One => 1,
                ConstObjClass::Ones => 0xFF,
                ConstObjClass::Zero => 0,
            }
        )
    }
}
