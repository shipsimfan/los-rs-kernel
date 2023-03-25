use super::{impl_core_display, ArgObj, Display, LocalObj, NameString, Result, Stream};

pub(super) enum SimpleName {
    ArgObj(ArgObj),
    LocalObj(LocalObj),
    NameString(NameString),
}

impl SimpleName {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match LocalObj::parse(stream)? {
            Some(local_obj) => return Ok(SimpleName::LocalObj(local_obj)),
            None => {}
        }

        match ArgObj::parse(stream)? {
            Some(arg_obj) => Ok(SimpleName::ArgObj(arg_obj)),
            None => {
                NameString::parse(stream).map(|name_string| SimpleName::NameString(name_string))
            }
        }
    }
}

impl Display for SimpleName {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            SimpleName::ArgObj(arg_obj) => arg_obj.display(f, depth, last),
            SimpleName::LocalObj(local_obj) => local_obj.display(f, depth, last),
            SimpleName::NameString(name_string) => {
                self.display_prefix(f, depth)?;
                write!(f, "{}", name_string)
            }
        }
    }
}

impl_core_display!(SimpleName);
