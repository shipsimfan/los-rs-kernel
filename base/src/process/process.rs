use super::Thread;
use crate::{
    util::{Map, Mappable, MappableMut},
    Mutex,
};
use alloc::{borrow::Cow, sync::Weak};

pub struct Process {
    id: u64,
    name: Cow<'static, str>,

    //address_space: AddressSpace,

    // TODO: Upgrade to RWLock
    threads: Mutex<Map<u64, (u64, Weak<Thread>)>>,
}

impl Process {
    pub(super) fn new<S: Into<Cow<'static, str>>>(name: S) -> Self {
        Process {
            id: 0,
            name: name.into(),

            threads: Mutex::new(Map::new(0)),
        }
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}

impl Mappable<u64> for Process {
    fn id(&self) -> &u64 {
        &self.id
    }
}

impl MappableMut<u64> for Process {
    fn set_id(&mut self, id: &u64) {
        self.id = *id;
    }
}
