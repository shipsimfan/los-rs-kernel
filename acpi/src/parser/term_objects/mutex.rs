use crate::parser::{next, NameString, Result, Stream};

pub(crate) struct Mutex {
    name: NameString,
    sync_level: u8,
}

impl Mutex {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let sync_level = next!(stream);

        Ok(Mutex { name, sync_level })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn sync_level(&self) -> u8 {
        self.sync_level
    }
}
