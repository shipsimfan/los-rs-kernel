pub(crate) struct Timer;

impl core::fmt::Display for Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Timer")
    }
}
