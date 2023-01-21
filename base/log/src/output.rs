use critical::{CriticalLock, LocalState};
use io::Write;

pub trait LogOutput: 'static {
    fn write(&self, output: &str);
}

impl<T: Write + 'static, L: LocalState> LogOutput for CriticalLock<T, L> {
    fn write(&self, output: &str) {
        self.lock().write(output.as_bytes()).ok();
    }
}
