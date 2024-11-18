use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};

use cache::{ShareableCache, SizeLimitedCache};
use dashmap::DashMap;

struct Entry<K, V> {
    lock: RwLock<Option<(Arc<K>, V)>>,
    read: AtomicBool,
}

struct EvictState {
    hand: usize,
}

pub struct ParallelSieveCache<K, V> {
    entries: Vec<Entry<K, V>>,
    map: DashMap<Arc<K>, usize>,
    evict: Mutex<EvictState>,
}

impl<K, V> ParallelSieveCache<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::with_capacity(cache::MAX_SIZE)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::from_iter((0..capacity).map(|_| Entry {
                lock: RwLock::new(None),
                read: AtomicBool::new(false),
            })),
            map: DashMap::with_capacity(capacity),
            evict: Mutex::new(EvictState { hand: 0 }),
        }
    }

    fn evict(&self) -> (usize, RwLockWriteGuard<Option<(Arc<K>, V)>>) {
        let mut state = self.evict.lock().unwrap_or_else(|e| {
            self.evict.clear_poison();

            e.into_inner()
        });

        loop {
            let index = state.hand;

            state.hand += 1;
            if state.hand >= self.entries.len() {
                state.hand = 0;
            }

            let entry = &self.entries[index];
            let read = entry.read.swap(false, Ordering::Relaxed);
            if read {
                continue;
            }

            let mut lock = entry.lock.write().unwrap_or_else(|e| {
                entry.lock.clear_poison();

                e.into_inner()
            });

            if let Some((key, _value)) = lock.take() {
                // Only remove the map entry if it points at the one we just removed.
                self.map.remove_if(&key, |_, &value| value == index);
            }

            break (index, lock);
        }
    }
}

impl<K, V> ShareableCache<K, V> for ParallelSieveCache<K, V>
where
    K: Send + Sync + Hash + Eq,
    V: Send + Sync + Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        let index = *self.map.get(key)?;
        let entry = &self.entries[index];
        let lock = entry.lock.read().expect("mutex was poisoned");

        entry.read.store(true, Ordering::Relaxed);
        let (_, value) = lock.as_ref()?;

        Some(value.clone())
    }

    fn set(&self, key: K, value: V) {
        let key = Arc::new(key);
        let (index, mut slot) = self.evict();

        *slot = Some((key.clone(), value));
        self.map.insert(key, index);
    }
}

impl<K, V> SizeLimitedCache<K, V> for ParallelSieveCache<K, V>
where
    K: Send + Sync + Hash + Eq,
    V: Send + Sync + Clone,
{
    fn get(&mut self, key: &K) -> Option<V> {
        ShareableCache::get(self, key)
    }

    fn set(&mut self, key: K, value: V) {
        ShareableCache::set(self, key, value)
    }
}
