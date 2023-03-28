macro_rules! next {
    ($stream: expr) => {{
        let offset = $stream.offset();
        $stream
            .next()
            .ok_or($crate::parser::Error::unexpected_end_of_stream(offset))
            .unwrap()
    }};
}

macro_rules! match_next {
    ($stream: expr, $($pattern: pat => $result: expr)*) => {{
        let offset = $stream.offset();
        match $crate::parser::next!($stream) {
            $($pattern => $result,)*
            c => return Err($crate::parser::Error::unexpected_byte(c, offset)).unwrap(),
        }}
    };
}

pub(super) use {match_next, next};
