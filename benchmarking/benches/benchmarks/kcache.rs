use std::hash::RandomState;

use cache::{ShareableCache, SizeLimitedCache};

pub struct KCache<Key, Value>(k_cache::Cache<Key, Value, RandomState>);

impl<Key, Value> KCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
{
    pub fn new() -> Self {
        Self(k_cache::Cache::new(RandomState::new(), cache::MAX_SIZE))
    }
}

impl<Key, Value> SizeLimitedCache<Key, Value> for KCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        self.0.get(key).cloned()
    }

    fn set(&mut self, key: Key, value: Value) {
        self.0.put(key, value);
    }
}

/// An internally-shareable cache that implements "internal mutability."
pub struct SharableKCache<Key, Value>(k_cache::SegmentedCache<Key, Value, RandomState>);

impl<Key, Value> SharableKCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
{
    pub fn new() -> Self {
        Self(k_cache::SegmentedCache::new(16, cache::MAX_SIZE))
    }
}

impl<Key, Value> ShareableCache<Key, Value> for SharableKCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone + Send,
    Value: Clone + Send,
{
    fn get(&self, key: &Key) -> Option<Value> {
        self.0.get(key)
    }

    fn set(&self, key: Key, value: Value) {
        self.0.put(key, value);
    }
}
