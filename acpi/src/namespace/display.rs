pub(crate) trait Display: core::fmt::Display {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result;
}

macro_rules! impl_core_display {
    ($type: ident) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.display(f, 0, true)
            }
        }
    };
}

pub(super) use impl_core_display;
