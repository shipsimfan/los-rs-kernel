use core::fmt::Arguments;

pub trait LogOutput {
    fn write_str(&self, s: &str);
    fn write_fmt(&self, args: Arguments<'_>);
}
