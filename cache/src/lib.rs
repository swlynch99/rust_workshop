mod cache_trait;
mod shareable_cache;

/// The policy for the basic workshop cache is just based on size.
pub const MAX_SIZE: usize = 100;

pub use cache_trait::SizeLimitedCache;
pub use shareable_cache::{synchronized_cache, ShareableCache, SynchronizedShareableCache};
