mod local;
mod lock;
mod ref_cell;

pub(crate) use local::CriticalState;

pub use local::CriticalKey;
pub use lock::*;
pub use ref_cell::CriticalRefCell;
