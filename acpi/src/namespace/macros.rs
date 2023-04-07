macro_rules! display_prefix {
    ($f: expr, $depth: expr) => {
        for _ in 0..$depth {
            write!($f, "  ")?;
        }
    };
}

macro_rules! display_name {
    ($f: expr, $name: expr) => {
        for byte in $name {
            write!($f, "{}", byte as char)?;
        }
    };
}

pub(super) use display_prefix;

pub(crate) use display_name;
