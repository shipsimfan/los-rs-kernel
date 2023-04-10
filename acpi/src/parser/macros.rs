macro_rules! next {
    ($stream: expr, $source: expr) => {{
        let offset = $stream.offset();
        $stream
            .next()
            .ok_or($crate::parser::Error::unexpected_end_of_stream(
                offset, $source,
            ))?
    }};
}

macro_rules! match_next {
    ($stream: expr, $source: expr $(,$pattern: pat => $result: expr)*,) => {{
        let offset = $stream.offset();
        match $crate::parser::next!($stream, $source) {
            $($pattern => $result,)*
            c => return Err($crate::parser::Error::unexpected_byte(c, offset, $source)),
        }}
    };
}

pub(super) use {match_next, next};
