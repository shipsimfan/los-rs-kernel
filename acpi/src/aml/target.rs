use super::{impl_core_display, peek, super_name::SuperName, Display, Result, Stream};

pub(super) enum Target {
    NullName,
    SuperName(SuperName),
}

impl Target {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match peek!(stream) {
            0x00 => {
                stream.next();
                Ok(Target::NullName)
            }
            _ => SuperName::parse(stream).map(|super_name| Target::SuperName(super_name)),
        }
    }
}

impl Display for Target {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Target::NullName => Ok(()),
            Target::SuperName(super_name) => super_name.display(f, depth, last),
        }
    }
}

impl_core_display!(Target);
