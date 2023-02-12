use io::Write;
use local_state::critical::CriticalLock;

pub trait LogOutput<'local>: 'local {
    fn write(&self, output: &str);
}

impl<'local, T: Write + 'static> LogOutput<'local> for CriticalLock<T> {
    fn write(&self, output: &str) {
        self.lock().write(output.as_bytes()).ok();
    }
}
