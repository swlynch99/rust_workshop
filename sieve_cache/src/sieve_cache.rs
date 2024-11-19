use std::collections::HashMap;
use std::hash::Hash;

use cache::SizeLimitedCache;
use slab::Slab;

struct Entry<K, V> {
    key: K,
    value: V,
    read: bool,
}

pub struct SieveCache<Key, Value> {
    entries: Slab<Entry<Key, Value>>,
    map: HashMap<Key, usize>,
    hand: usize,
}

impl<Key, Value> SieveCache<Key, Value>
where
    Key: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            entries: Slab::with_capacity(cache::MAX_SIZE),
            map: HashMap::new(),
            hand: 0,
        }
    }

    /// Evict entries until the cache goes below `cache::MAX_SIZE`.
    ///
    /// This method does nothing if the cache is already below `cache::MAX_SIZE`.
    fn evict(&mut self) {
        while self.entries.len() >= cache::MAX_SIZE {
            if let Some(entry) = self.entries.get_mut(self.hand) {
                if !std::mem::take(&mut entry.read) {
                    self.map.remove(&entry.key);
                    self.entries.remove(self.hand);
                }
            }

            self.hand += 1;
            if self.hand >= cache::MAX_SIZE {
                self.hand = 0;
            }
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
        let index = *self.map.get(key)?;
        let entry = &mut self.entries[index];

        entry.read = true;

        Some(entry.value.clone())
    }

    fn set(&mut self, key: Key, value: Value) {
        if let Some(&index) = self.map.get(&key) {
            self.entries[index] = Entry {
                key,
                value,
                read: false,
            };

            return;
        }

        self.evict();
        let index = self.entries.insert(Entry {
            key: key.clone(),
            value,
            read: false,
        });
        self.map.insert(key, index);
    }
}

impl<Key, Value> Default for SieveCache<Key, Value>
where
    Key: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}
