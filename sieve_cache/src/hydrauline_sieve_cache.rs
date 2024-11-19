use std::hash::Hash;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use cache::{ShareableCache, SizeLimitedCache};
use dashmap::DashMap;
use seize::{Collector, Guard, Linked};

struct Entry<K, V> {
    data: AtomicPtr<Linked<(Arc<K>, V)>>,
    read: AtomicBool,
    mutex: Mutex<()>,
}

impl<K, V> Entry<K, V> {
    fn new() -> Self {
        Self {
            data: AtomicPtr::new(std::ptr::null_mut()),
            read: AtomicBool::new(false),
            mutex: Mutex::new(()),
        }
    }
}

pub struct HydraulineSieveCache<K, V> {
    collector: Collector,

    map: DashMap<Arc<K>, usize>,
    entries: Vec<Entry<K, V>>,
    hand: AtomicUsize,
}

impl<K, V> HydraulineSieveCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self::with_capacity(cache::MAX_SIZE)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        assert_ne!(capacity, 0);

        Self {
            collector: Collector::new(),
            map: DashMap::with_capacity(capacity),
            entries: {
                let mut entries = Vec::with_capacity(capacity);
                entries.resize_with(capacity, Entry::new);
                entries
            },
            hand: AtomicUsize::new(0),
        }
    }

    fn evict_slot(&self) -> (usize, MutexGuard<()>) {
        loop {
            let current = fetch_inc_mod(&self.hand, self.entries.len(), Ordering::Relaxed);
            let entry = &self.entries[current];

            if entry.read.swap(false, Ordering::Relaxed) {
                continue;
            }

            let lock = entry.mutex.lock().unwrap_or_else(|e| {
                entry.mutex.clear_poison();

                e.into_inner()
            });

            break (current, lock);
        }
    }
}

impl<K, V> ShareableCache<K, V> for HydraulineSieveCache<K, V>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync + Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        let guard = self.collector.enter();
        let index = *self.map.get(key)?;

        let entry = &self.entries[index];

        let data = guard.protect(&entry.data, Ordering::Acquire);
        if data.is_null() {
            return None;
        }

        entry.read.store(true, Ordering::Relaxed);

        let (k, v) = unsafe { &(*data).value };

        // A value may be in the process of being inserted so we got an entry for a different key.
        if key != &**k {
            return None;
        }

        Some(v.clone())
    }

    fn set(&self, key: K, value: V) {
        let key = Arc::new(key);
        let data = self.collector.link_boxed((key.clone(), value));
        let (index, _lock) = self.evict_slot();

        let guard = self.collector.enter();
        let entry = &self.entries[index];

        self.map.insert(key, index);
        let prev = entry.data.swap(data, Ordering::AcqRel);

        if !prev.is_null() {
            let data = unsafe { &(*prev).value };
            self.map.remove(&data.0);

            unsafe { guard.defer_retire(prev, seize::reclaim::boxed::<Linked<(Arc<K>, V)>>) };
        }
    }
}
impl<K, V> SizeLimitedCache<K, V> for HydraulineSieveCache<K, V>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync + Clone,
{
    fn get(&mut self, key: &K) -> Option<V> {
        ShareableCache::get(self, key)
    }

    fn set(&mut self, key: K, value: V) {
        ShareableCache::set(self, key, value)
    }
}

fn fetch_inc_mod(data: &AtomicUsize, modulus: usize, success: Ordering) -> usize {
    assert!(modulus != 0);

    if modulus == 1 {
        return data.swap(0, success);
    }

    let mut previous = data.load(Ordering::Relaxed);
    loop {
        let mut next = previous + 1;
        if next >= modulus {
            next -= modulus;
        }

        match data.compare_exchange_weak(previous, next, success, Ordering::Relaxed) {
            Ok(v) => break v % modulus,
            Err(prev) => previous = prev,
        }
    }
}
