macro_rules! set_bit {
    ($field: expr, $bit: expr) => {
        $field |= (1 << $bit);
    };
}

macro_rules! clear_bit {
    ($field: expr, $bit: expr) => {
        $field &= !(1 << $bit)
    };
}

macro_rules! check_bit {
    ($field: expr, $bit: expr) => {
        $field & (1 << $bit) == (1 << $bit)
    };
}

pub(super) use {check_bit, clear_bit, set_bit};
