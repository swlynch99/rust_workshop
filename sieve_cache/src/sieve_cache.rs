use cache::OneHundredItemCache;

pub struct SieveCache {}

impl<Key, Value> OneHundredItemCache<Key, Value> for SieveCache
// See the comment on SieveCache for commentary on "where clauses" in rust.
where
    Key: Clone + Eq + std::hash::Hash,
    Value: Clone,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        todo!()
    }

    fn set(&mut self, key: Key, value: Value) {
        todo!()
    }
}
