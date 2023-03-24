use super::{
    impl_core_display,
    objects::{Object, Scope},
    Display,
};

pub(crate) struct Namespace {
    root: Object,
}

const PREDEFINED_SCOPES: &[[u8; 4]] = &[*b"_GPE", *b"_PR_", *b"_SB_", *b"_SI_", *b"_TZ_"];

impl Namespace {
    pub(crate) fn new() -> Self {
        let mut root = Scope::new(None);

        for name in PREDEFINED_SCOPES {
            root.add_child(Object::Scope(Scope::new(Some(*name))))
                .unwrap()
        }

        Namespace {
            root: Object::Scope(root),
        }
    }

    pub(crate) fn get_mut(&mut self, path: &[[u8; 4]]) -> Option<&mut Object> {
        self.root.get_child_mut(path)
    }
}

impl Display for Namespace {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.root.display(f, depth)
    }
}

impl_core_display!(Namespace);
