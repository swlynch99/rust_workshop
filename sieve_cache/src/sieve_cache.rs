use cache::SizeLimitedCache;

pub struct SieveCache<Key, Value> {
    /// This is a placeholder to allow the code to compile in a work-in-progress state.
    /// You'll remove this field when you choose a data structure to hold the raw cache
    /// values.
    _phantom: std::marker::PhantomData<(Key, Value)>,
}

impl<Key, Value> SieveCache<Key, Value> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Key, Value> SizeLimitedCache<Key, Value> for SieveCache<Key, Value>
// See the comment on SieveCache for commentary on "where clauses" in rust.
where
    Key: Clone + Eq + std::hash::Hash,
    Value: Clone,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        // todo!()
        None
    }

    fn set(&mut self, key: Key, value: Value) {
        // todo!()
    }
}
