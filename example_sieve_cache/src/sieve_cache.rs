use std::{collections::HashMap, hash::Hash};

use cache::{SizeLimitedCache, MAX_SIZE};

use crate::nodes::{new_reference_pair, ReferenceNode, ValueNode};

// This struct is generic over the key and the value types. In rust, if you want a struct to be
// generic, something inside the struct must depend on those generics. No useless generics allowed,
// by default.
pub struct SieveCache<Key, Value> {
    cache: HashMap<Key, ValueNode<Value>>,
    sieve_list: Vec<ReferenceNode<Key>>,
    hand_index: usize,
}

// an implementation must be generic to implement something generically. If you're familiar with c++ this is
// probably familiar to your template specialization mind, but otherwise it's a little odd at first.
// You don't simply declare that something is generic - you provide how it is generic and what the rules are.
//
// This is the implementation of the SizeLimitedCache adapter trait for the example SieveCache struct.
impl<Key, Value> SizeLimitedCache<Key, Value> for SieveCache<Key, Value>
// See the comment on the SizeLimitedCache trait for commentary on "where clauses" in rust.
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        match self.cache.get(key) {
            Some(node) => {
                node.set_read();
                Some(node.value().clone())
            }
            None => None,
        }
    }

    fn set(&mut self, key: Key, value: Value) {
        self.make_room_for_one_insertion();

        self.insert_new_pair(key, value);
    }
}

// This is the implementation of the SieveCache struct itself. Rust breaks up your trait implementations
// and struct implementations into separate blocks. It's a nice opportunity to organize your code.
impl<Key, Value> SieveCache<Key, Value>
// These constraints help simplify the generic types on the cache and sieve list.
// This isn't strictly optimal, but it's a good starting point.
where
    Key: Eq + Hash + Clone,
    Value: Clone,
{
    /// Create a new example SieveCache.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            sieve_list: Vec::new(),
            hand_index: 0,
        }
    }

    /// Makes sure the cache doesn't exceed cache::MAX_SIZE - 1 items, so that one more item can be inserted.
    fn make_room_for_one_insertion(&mut self) {
        while !self.sieve_list.is_empty() && MAX_SIZE <= self.sieve_list.len() {
            let node = &self.sieve_list[self.hand_index];

            let node_has_been_read_since_last_time_the_hand_checked_it = node.take_read_state();
            if node_has_been_read_since_last_time_the_hand_checked_it {
                // move on to the next node until we find one that hasn't been read
                self.hand_index = (self.hand_index + 1) % self.sieve_list.len();
            } else {
                // here's a probably-useless item: Remove it
                self.cache.remove(node.key());
                self.sieve_list.remove(self.hand_index);
                self.hand_index %= self.sieve_list.len();
            }
        }
    }

    /// Inserts a new key-value pair into the cache and sieve list.
    fn insert_new_pair(&mut self, key: Key, value: Value) {
        let (reference, value) = new_reference_pair(key.clone(), value);
        self.sieve_list.push(reference);
        self.cache.insert(key, value);
    }
}

/// Implementing Default for a struct is a Rust-ism that basically means "default constructor"
/// if you're coming from Java or similar.
/// You will often see things like `SieveCache::default()`, `Default::default()`, and
/// `#[derive(Default)]`. The first 2 are ways to call this default() function, and the 3rd is a
/// macro that declares your type as being composed of Default-types.
/// It's good to implement it when your type has a sensible default!
impl<Key, Value> Default for SieveCache<Key, Value>
where
    Key: Eq + Hash + Clone,
    Value: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use cache::{SizeLimitedCache, MAX_SIZE};

    use crate::SieveCache;

    #[test]
    fn one() {
        let mut cache = SieveCache::new();
        cache.set(1, 1);
        assert_eq!(cache.get(&1), Some(1));
    }

    #[test]
    fn full_cache() {
        let mut cache = SieveCache::new();
        for i in 0..MAX_SIZE {
            cache.set(i, i);
        }
        for i in 0..MAX_SIZE {
            assert_eq!(cache.get(&i), Some(i), "all the elements should be there");
        }

        cache.set(MAX_SIZE, MAX_SIZE);
        assert_eq!(
            cache.get(&MAX_SIZE),
            Some(MAX_SIZE),
            "the new element should be there"
        );
        let count = (0..=MAX_SIZE).filter_map(|i| cache.get(&i)).count();
        assert_eq!(count, MAX_SIZE, "the cache should have evicted one element");
    }
}
