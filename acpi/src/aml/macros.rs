macro_rules! next {
    ($stream: expr) => {{
        let offset = $stream.offset();
        $stream
            .next()
            .ok_or($crate::aml::Error::unexpected_end_of_stream(offset))?
    }};
}

macro_rules! match_next {
    ($stream: expr, $($pattern: pat => $result: expr)*) => {{
        let offset = $stream.offset();
        let c = $crate::aml::next!($stream);
        match c {
            $($pattern => $result,)*
            _ => return Err($crate::aml::Error::unexpected_byte(offset, c)),
        }}
    };
}

macro_rules! peek {
    ($stream: expr) => {{
        $stream
            .peek()
            .ok_or($crate::aml::Error::unexpected_end_of_stream(
                $stream.offset(),
            ))?
    }};
}

macro_rules! match_peek {
    ($stream: expr, $($pattern: pat => $result: expr)*) => {{
        let c = $crate::aml::peek!($stream);
        match c {
            $($pattern => $result,)*
            _ => return Err($crate::aml::Error::unexpected_byte($stream.offset(), c)),
        }}
    };
}

pub(super) use {match_next, match_peek, next, peek};
