use super::NameString;
use crate::parser::{Result, Stream};

pub(super) enum SimpleName {
    //ArgObj(ArgObj),
    //LocalObj(LocalObj),
    NameString(NameString),
}

impl SimpleName {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        /*match LocalObj::parse(stream)? {
            Some(local_obj) => return Ok(SimpleName::LocalObj(local_obj)),
            None => {}
        }*/

        /*match ArgObj::parse(stream)? {
        Some(arg_obj) => Ok(SimpleName::ArgObj(arg_obj)),
        None => {*/
        NameString::parse(stream).map(|name_string| SimpleName::NameString(name_string))
        //  }
        //}
    }
}
