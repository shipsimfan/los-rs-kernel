use critical::{CriticalLock, LocalState};
use io::Write;

pub trait LogOutput<'local>: 'local {
    fn write(&self, output: &str);
}

impl<'local, T: Write + 'static, L: LocalState + 'local> LogOutput<'local> for CriticalLock<T, L> {
    fn write(&self, output: &str) {
        self.lock().write(output.as_bytes()).ok();
    }
}
