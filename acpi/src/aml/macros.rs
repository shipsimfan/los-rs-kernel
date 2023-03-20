macro_rules! next {
    ($stream: expr) => {
        match $stream.next() {
            Some(c) => c,
            None => return Err(crate::aml::Error::UnexpectedEndOfStream),
        }
    };
}

pub(super) use next;
