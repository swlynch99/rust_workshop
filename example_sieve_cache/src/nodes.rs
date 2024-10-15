use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Helper struct for value reference tracker - goes in the cache map
pub struct ValueNode<Value> {
    value: Value,
    read: Arc<AtomicBool>,
}

/// Helper struct for value reference tracker - goes in the sieve list to reference the cache map
pub struct ReferenceNode<Key> {
    key: Key,
    read: Arc<AtomicBool>,
}

pub fn new_reference_pair<Key, Value>(
    key: Key,
    value: Value,
) -> (ReferenceNode<Key>, ValueNode<Value>) {
    let read = Arc::new(AtomicBool::new(false));
    (
        ReferenceNode {
            key,
            read: read.clone(),
        },
        ValueNode { value, read },
    )
}

impl<Key> ReferenceNode<Key> {
    pub fn take_read_state(&self) -> bool {
        self.read.swap(false, Ordering::Relaxed)
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl<Value> ValueNode<Value> {
    pub fn set_read(&self) {
        self.read.store(true, Ordering::Relaxed)
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
