use super::Object;
use crate::aml::Result;
use crate::namespace::{impl_core_display, Display};
use alloc::vec::Vec;

pub(crate) struct Scope {
    name: Option<[u8; 4]>,
    objects: Vec<Object>,
}

impl Scope {
    pub(crate) fn new(name: Option<[u8; 4]>) -> Self {
        Scope {
            name,
            objects: Vec::new(),
        }
    }

    pub(super) fn name(&self) -> Option<[u8; 4]> {
        self.name
    }

    pub(super) fn get_child(&self, path: &[[u8; 4]]) -> Option<&Object> {
        assert!(path.len() > 0);

        for object in &self.objects {
            if let Some(name) = object.name() {
                if name == path[0] {
                    return object.get_child(if path.len() == 1 { &[] } else { &path[1..] });
                }
            }
        }

        None
    }

    pub(super) fn get_child_mut(&mut self, path: &[[u8; 4]]) -> Option<&mut Object> {
        assert!(path.len() > 0);

        for object in &mut self.objects {
            if let Some(name) = object.name() {
                if name == path[0] {
                    return object.get_child_mut(if path.len() == 1 { &[] } else { &path[1..] });
                }
            }
        }

        None
    }

    pub(in crate::namespace) fn add_child(&mut self, object: Object) -> Result<()> {
        let name = object.name();
        if let Some(name) = name {
            for object in &self.objects {
                if let Some(object_name) = object.name() {
                    if object_name == name {
                        return Err(crate::aml::Error::NameCollision(name));
                    }
                }
            }
        }

        Ok(self.objects.push(object))
    }
}

impl Display for Scope {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Scope")?;
        self.display_name(f, self.name)?;
        writeln!(f, ":")?;

        for object in &self.objects {
            object.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl_core_display!(Scope);
