use crate::{
    parser::{name_string, Context, Result, Stream, SuperName},
    Path,
};

pub(crate) struct Load<'a> {
    path: Path,
    target: SuperName<'a>,
}

impl<'a> Load<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let path = name_string::parse(stream, "Load")?;
        let target = SuperName::parse(stream, context)?;

        Ok(Load { path, target })
    }
}

impl<'a> core::fmt::Display for Load<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Load ({}, {})", self.path, self.target)
    }
}
