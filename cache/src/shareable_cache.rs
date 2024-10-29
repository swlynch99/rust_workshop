use std::sync::Mutex;

use crate::SizeLimitedCache;

/// Describes a cache that can be shared between threads.
pub trait ShareableCache<Key, Value>: Send + Sync
where
    Key: Eq + std::hash::Hash,
    Value: Clone,
{
    /// Gets a value from the cache, or None if it is not present (or has been evicted).
    ///
    /// Note that the only difference between this and the SizeLimitedCache trait is that this
    /// trait does not require a mutable reference to self. That makes it easier to use, but harder
    /// to implement!
    fn get(&self, key: &Key) -> Option<Value>;

    /// Sets a value in the cache.
    fn set(&self, key: Key, value: Value);
}

pub struct SynchronizedShareableCache<Cache> {
    /// One way to get mutable ownership of a value is to wrap it in a Mutex.
    /// There are other ways, and you can experiment with them if you like.
    /// For example, you might try benchmarking RwLock, and see if your assumptions
    /// match reality!
    ///
    /// cache: std::sync::RwLock<Cache>,
    cache: Mutex<Cache>,
}

/// Wraps a cache in a mutex, making it shareable between threads.
pub fn synchronized_cache<Cache, Key, Value>(cache: Cache) -> SynchronizedShareableCache<Cache>
where
    Key: Eq + std::hash::Hash,
    Value: Clone,
    Cache: SizeLimitedCache<Key, Value>,
{
    SynchronizedShareableCache {
        cache: Mutex::new(cache),
    }
}

impl<Cache, Key, Value> ShareableCache<Key, Value> for SynchronizedShareableCache<Cache>
where
    Key: Eq + std::hash::Hash,
    Value: Clone,
    Cache: SizeLimitedCache<Key, Value> + Send,
{
    fn get(&self, key: &Key) -> Option<Value> {
        self.cache.lock().expect("mutex should work").get(key)
    }

    fn set(&self, key: Key, value: Value) {
        self.cache
            .lock()
            .expect("mutex should work")
            .set(key, value);
    }
}
