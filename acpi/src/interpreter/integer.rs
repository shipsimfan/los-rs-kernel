pub(crate) enum Integer {
    ACPI1(u32),
    ACPI2(u64),
}

impl core::fmt::Display for Integer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Integer::ACPI1(value) => value.fmt(f),
            Integer::ACPI2(value) => value.fmt(f),
        }
    }
}

impl Into<usize> for Integer {
    fn into(self) -> usize {
        match self {
            Integer::ACPI1(value) => value as usize,
            Integer::ACPI2(value) => value as usize,
        }
    }
}
