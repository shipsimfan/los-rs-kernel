pub(super) trait Display {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result;

    fn display_prefix(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        for _ in 0..depth {
            write!(f, "  ")?;
        }
        Ok(())
    }

    fn display_name(
        &self,
        f: &mut core::fmt::Formatter,
        name: Option<[u8; 4]>,
    ) -> core::fmt::Result {
        if let Some(name) = name {
            write!(
                f,
                " \"{}{}{}{}\"",
                name[0] as char, name[1] as char, name[2] as char, name[3] as char
            )
        } else {
            Ok(())
        }
    }
}

macro_rules! impl_core_display {
    ($typename: ident) => {
        impl core::fmt::Display for $typename {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.display(f, 0)
            }
        }
    };
}

pub(super) use impl_core_display;
