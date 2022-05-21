mod owner;
mod reference;

pub use owner::Owner;
pub use reference::Reference;

pub trait Lock: Send + Sync {
    type Data;

    fn new(data: Self::Data) -> Self;
    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
}
