pub trait Increment {
    fn increment(&mut self);
}

impl Increment for usize {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for u64 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for u32 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for u16 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for u8 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for isize {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for i64 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for i32 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for i16 {
    fn increment(&mut self) {
        *self += 1;
    }
}

impl Increment for i8 {
    fn increment(&mut self) {
        *self += 1;
    }
}
