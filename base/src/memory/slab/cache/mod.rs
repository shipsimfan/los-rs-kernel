mod inner;
mod outer;

pub(in crate::memory) use inner::CacheInner;

pub use outer::Cache;

static CACHE_CACHE: CacheInner = CacheInner::new_type::<CacheInner>();
