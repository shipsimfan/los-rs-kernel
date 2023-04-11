use crate::{
    parser::{ArgObj, LocalObj, Result, Stream},
    Path,
};

use super::name_string;

pub(crate) enum SimpleName {
    Arg(ArgObj),
    Local(LocalObj),
    Path(Path),
}

impl SimpleName {
    pub(in crate::parser) fn parse(stream: &mut Stream) -> Result<Self> {
        if let Some(arg_obj) = ArgObj::parse(stream)? {
            return Ok(SimpleName::Arg(arg_obj));
        }

        if let Some(local_obj) = LocalObj::parse(stream)? {
            return Ok(SimpleName::Local(local_obj));
        }

        name_string::parse(stream, "Simple Name").map(|path| SimpleName::Path(path))
    }
}

impl core::fmt::Display for SimpleName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SimpleName::Arg(arg_obj) => arg_obj.fmt(f),
            SimpleName::Local(local_obj) => local_obj.fmt(f),
            SimpleName::Path(path) => path.fmt(f),
        }
    }
}
