mod local;
mod lock;

pub(crate) use local::CriticalState;

pub use local::CriticalKey;
pub use lock::*;
