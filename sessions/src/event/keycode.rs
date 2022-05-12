#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyState {
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
    pub left_shift: bool,
    pub right_shift: bool,
    pub left_ctrl: bool,
    pub right_ctrl: bool,
    pub left_alt: bool,
    pub right_alt: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Keycode {
    Undefined,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Pause,
    PrintScreen,
    Delete,
    Home,
    PageUp,
    PageDown,
    End,
    NumAsterick,
    NumMinus,
    NumPlus,
    NumPeriod,
    Insert,
    Space = b' ',
    Quote = b'\'',
    Comma = b',',
    Minus,
    Period,
    ForwardSlash,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    SemiColon = b';',
    Equal = b'=',
    Backspace,
    Tab,
    CapsLock,
    NumLock,
    ScrollLock,
    LeftShift,
    LeftControl,
    Function,
    Windows,
    LeftAlt,
    RightAlt,
    RightControl,
    RightShift,
    Enter,
    OpenSquareBracket = b'[',
    Backslash,
    CloseSquareBracket,
    Tick = b'`',
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
}

const STATE_CAPS_LOCK: usize = 0x001;
const STATE_NUM_LOCK: usize = 0x002;
const STATE_SCROLL_LOCK: usize = 0x004;
const STATE_LEFT_SHIFT: usize = 0x008;
const STATE_RIGHT_SHIFT: usize = 0x010;
const STATE_LEFT_CTRL: usize = 0x020;
const STATE_RIGHT_CTRL: usize = 0x040;
const STATE_LEFT_ALT: usize = 0x080;
const STATE_RIGHT_ALT: usize = 0x100;

impl KeyState {
    pub const fn new() -> Self {
        KeyState {
            caps_lock: false,
            num_lock: false,
            scroll_lock: false,
            left_shift: false,
            right_shift: false,
            left_ctrl: false,
            right_ctrl: false,
            left_alt: false,
            right_alt: false,
        }
    }
}

impl Into<usize> for KeyState {
    fn into(self) -> usize {
        let mut state = 0;

        if self.caps_lock {
            state |= STATE_CAPS_LOCK;
        }

        if self.num_lock {
            state |= STATE_NUM_LOCK;
        }

        if self.scroll_lock {
            state |= STATE_SCROLL_LOCK;
        }

        if self.left_shift {
            state |= STATE_LEFT_SHIFT;
        }

        if self.right_shift {
            state |= STATE_RIGHT_SHIFT;
        }

        if self.left_ctrl {
            state |= STATE_LEFT_CTRL;
        }

        if self.right_ctrl {
            state |= STATE_RIGHT_CTRL;
        }

        if self.left_alt {
            state |= STATE_LEFT_ALT;
        }

        if self.right_alt {
            state |= STATE_RIGHT_ALT;
        }

        state
    }
}
