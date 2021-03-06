#[allow(dead_code)]
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Multiset<K: Eq + Hash>(HashMap<K, usize>);

impl<K> Multiset<K>
where
    K: Eq + Hash,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { 0: HashMap::new() }
    }

    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key: K) {
        match self.0.entry(key) {
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Vacant(entry) => {
                entry.insert(1);
            }
        }
    }

    #[allow(dead_code)]
    pub fn multiplicity(&self, key: &K) -> usize {
        *self.0.get(key).unwrap_or(&(0 as usize))
    }

    #[allow(dead_code)]
    pub fn inner(&self) -> &HashMap<K, usize> {
        &self.0
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> HashMap<K, usize> {
        self.0
    }
}

impl<K> Default for Multiset<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(HashMap::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_string() {
        let mut set: Multiset<String> = Multiset::new();
        for _ in 0..10 {
            set.insert("Hello".to_string());
        }

        assert_eq!(set.multiplicity(&"Hello".to_string()), 10);
        assert_eq!(set.multiplicity(&"Hallo".to_string()), 0);
    }

    #[test]
    fn insert_int() {
        let mut set = Multiset::new();

        for i in 0..100 {
            set.insert(i % 20);
        }

        assert_eq!(set.multiplicity(&0), 5);
        assert_eq!(set.multiplicity(&11), 5);
        assert_eq!(set.multiplicity(&19), 5);
    }
}
