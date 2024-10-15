/// Describes a cache that holds up to 100 items.
///
/// This is just a simple retention policy for workshop purposes.
pub trait OneHundredItemCache<Key, Value>
where
    // Keys in a cache must be able to be compared for equality, and they must be hashable.
    // You can add an Ord bound if you want to implement something like a BTreeMap cache.
    // For the purposes of this workshop, we'll stick with hash maps by default!
    Key: Eq + std::hash::Hash,
    Value: Clone,
{
    /// Gets a value from the cache, or None if it is not present (or has been evicted).
    fn get(&mut self, key: &Key) -> Option<Value>;

    /// Sets a value in the cache.
    fn set(&mut self, key: Key, value: Value);
}
