use super::{name_objects::name_string::Prefix, Error, Result};
use crate::namespace::{Namespace, Object};
use alloc::vec::Vec;

#[derive(Clone)]
pub(super) struct Context {
    path: Vec<[u8; 4]>,
}

impl Context {
    pub(super) fn new() -> Self {
        Context { path: Vec::new() }
    }

    pub(super) fn get_object<'a>(&self, namespace: &'a mut Namespace) -> Result<&'a mut Object> {
        namespace
            .get_mut(&self.path)
            .ok_or(Error::missing_name(self.path.get(0).map(|path| *path)))
    }

    pub(super) fn move_down(
        &mut self,
        prefix: Prefix,
        path: &[[u8; 4]],
        r#final: Option<[u8; 4]>,
        namespace: &mut Namespace,
    ) -> Result<()> {
        match prefix {
            Prefix::Root => *self = Context::new(),
            Prefix::Super(count) => self.move_up(count),
            Prefix::None => return self.no_prefix_move_down(path, r#final, namespace),
        }

        let current_object = self.get_object(namespace)?;

        match current_object.get_child(path) {
            Some(current_object) => {
                self.path.extend(path);
                match r#final {
                    Some(path) => {
                        current_object
                            .get_child(&[path])
                            .ok_or(Error::missing_name(r#final))?;
                        self.path.push(path);
                    }
                    None => {}
                }
            }
            None => return Err(Error::missing_name(r#final)),
        }

        Ok(())
    }

    fn move_up(&mut self, count: usize) {
        if count >= self.path.len() {
            self.path.clear();
        } else {
            for _ in 0..count {
                self.path.pop();
            }
        }
    }

    fn no_prefix_move_down(
        &mut self,
        path: &[[u8; 4]],
        r#final: Option<[u8; 4]>,
        namespace: &mut Namespace,
    ) -> Result<()> {
        for _ in 0..self.path.len() + 1 {
            match self.move_down(Prefix::Super(0), path, r#final, namespace) {
                Ok(()) => return Ok(()),
                Err(_) => self.move_up(1),
            }
        }

        Err(Error::MissingName(r#final))
    }
}
