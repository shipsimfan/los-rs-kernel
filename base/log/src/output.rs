pub trait LogOutput: 'static {
    fn write(&self, output: &str);
}
