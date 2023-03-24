macro_rules! impl_core_display {
    ($typename: ident) => {
        impl core::fmt::Display for $typename {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.display(f, 0)
            }
        }
    };
}

macro_rules! next {
    ($stream: expr) => {{
        let offset = $stream.offset();
        $stream
            .next()
            .ok_or($crate::aml::Error::unexpected_end_of_stream(offset))
            .unwrap()
    }};
}

macro_rules! match_next {
    ($stream: expr, $($pattern: pat => $result: expr)*) => {{
        let offset = $stream.offset();
        let c = $crate::aml::next!($stream);
        match c {
            $($pattern => $result,)*
            _ => return Err($crate::aml::Error::unexpected_byte(c, offset)).unwrap(),
        }}
    };
}

macro_rules! peek {
    ($stream: expr) => {{
        $stream
            .peek()
            .ok_or($crate::aml::Error::unexpected_end_of_stream(
                $stream.offset(),
            ))
            .unwrap()
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

macro_rules! unwrap_data_type {
    ($data_type: expr, $expected: ident) => {
        match $data_type {
            $crate::namespace::DataType::$expected(value) => Ok(value),
            _ => Err($crate::aml::Error::InvalidArgumentType),
        }
    };
}

pub(super) use {impl_core_display, match_next, match_peek, next, peek, unwrap_data_type};
