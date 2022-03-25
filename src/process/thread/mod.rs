mod inner;
mod owner;
mod queue;
mod reference;
mod stack;

pub use inner::ThreadInner;
pub use owner::ThreadOwner;
pub use queue::{AddFn, CurrentQueue, RemoveFn};
pub use reference::ThreadReference;

pub type ThreadFunc = fn() -> isize;
pub type ThreadFuncContext = fn(context: usize) -> isize;
