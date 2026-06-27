//SPDX:MIT

//! Utility handler for a hashmap storing a vector of objects per key.

use std::{
    collections::{HashMap, hash_map::Keys},
    hash::Hash,
};

pub struct HashMapVec<K: Eq + Hash, V> {
    inner: HashMap<K, Vec<V>>,
    empty: Vec<V>,
}

/// Stores a non-empty vector for a key.
impl<K: Eq + Hash, V> HashMapVec<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            empty: Vec::new(),
        }
    }

    pub fn keys(&self) -> Keys<'_, K, Vec<V>> {
        self.inner.keys()
    }

    pub fn contains_any(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    /// Add or replace the existing list at the key.
    pub fn replace(&mut self, key: K, list: Vec<V>) {
        if list.is_empty() {
            self.inner.remove(&key);
        } else {
            self.inner.insert(key, list);
        }
    }

    /// Insert a new value into the key.
    pub fn insert(&mut self, key: K, value: V) {
        match self.inner.get_mut(&key) {
            Some(v) => {
                v.push(value);
            }
            None => {
                let mut v = Vec::new();
                v.push(value);
                self.inner.insert(key, v);
            }
        }
    }

    /// Get the contents of the key as a reference.
    pub fn get_ref(&mut self, key: K) -> &Vec<V> {
        self.inner.get(&key).unwrap_or(&self.empty)
    }

    /// Remove the list at key
    pub fn handle(&mut self, key: K) -> Vec<V> {
        self.inner.remove(&key).unwrap_or(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let mut hmv = HashMapVec::new();
        hmv.insert(0, "foo".to_string());
        hmv.insert(0, "bar".to_string());
        let i = hmv.handle(0);
        let mut i = i.iter();
        assert_eq!(Some(&"foo".to_string()), i.next());
        assert_eq!(Some(&"bar".to_string()), i.next());
        assert_eq!(None, i.next());
    }
}
