use std::{collections::HashMap, hash::Hash};

use cache::OneHundredItemCache;

use crate::nodes::{new_reference_pair, ReferenceNode, ValueNode};

/// Since we're just implementing a cache with 100 items, we'll use a const to limit the size.
const MAX_SIZE: usize = 100;

pub struct SieveCache<Key, Value> {
    cache: HashMap<Key, ValueNode<Value>>,
    sieve_list: Vec<ReferenceNode<Key>>,
    hand: usize,
}

impl<Key, Value> OneHundredItemCache<Key, Value> for SieveCache<Key, Value>
// See the comment on the SieveCache trait for commentary on "where clauses" in rust.
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        if let Some(node) = self.cache.get(key) {
            node.set_read();
            Some(node.value().clone())
        } else {
            None
        }
    }

    fn set(&mut self, key: Key, value: Value) {
        self.make_room_for_one_insertion();

        self.insert_new_pair(key, value);
    }
}

impl<Key, Value> SieveCache<Key, Value>
// These constraints help simplify the generic types on the cache and sieve list.
// This isn't strictly optimal, but it's a good starting point.
where
    Key: Eq + Hash + Clone,
    Value: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            sieve_list: Vec::new(),
            hand: 0,
        }
    }

    fn make_room_for_one_insertion(&mut self) {
        while !self.sieve_list.is_empty() && MAX_SIZE <= self.sieve_list.len() {
            let node = &self.sieve_list[self.hand];
            let node_is_read = node.take_read_state();
            if !node_is_read {
                self.cache.remove(node.key());
                self.sieve_list.swap_remove(self.hand);
                self.hand %= self.sieve_list.len();
            } else {
                self.hand = (self.hand + 1) % self.sieve_list.len();
            }
        }
    }

    fn insert_new_pair(&mut self, key: Key, value: Value) {
        let (reference, value) = new_reference_pair(key.clone(), value);
        self.sieve_list.push(reference);
        self.cache.insert(key, value);
    }
}

#[cfg(test)]
mod test {
    use cache::OneHundredItemCache;

    use crate::SieveCache;

    use super::MAX_SIZE;

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
