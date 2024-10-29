use cache::{ShareableCache, SizeLimitedCache, MAX_SIZE};

pub struct LruCache<Key, Value>(moka::sync::Cache<Key, Value>);

impl<Key, Value> LruCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    Value: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self(moka::sync::Cache::new(MAX_SIZE as u64))
    }
}

impl<Key, Value> SizeLimitedCache<Key, Value> for LruCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    Value: Clone + Send + Sync + 'static,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        self.0.get(key)
    }

    fn set(&mut self, key: Key, value: Value) {
        self.0.insert(key, value);
    }
}

impl<Key, Value> ShareableCache<Key, Value> for LruCache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    Value: Clone + Send + Sync + 'static,
{
    fn get(&self, key: &Key) -> Option<Value> {
        self.0.get(key)
    }

    fn set(&self, key: Key, value: Value) {
        self.0.insert(key, value);
    }
}
