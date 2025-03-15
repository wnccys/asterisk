use std::{fmt::Error, hash::{Hash, Hasher}};
use crate::value::Value;
use super::hasher::FNV1aHasher;

#[derive(Debug)]
pub struct HashTable<K> {
    entries: Vec<Option<(K, Value)>>,
}

impl<K: Clone> Default for HashTable<K> {
    fn default() -> Self {
        Self {
            entries: vec![None; 4],
        }
    }
}

impl<K> HashTable<K> where K: Hash + Clone + Eq {
    const MAX_LOAD_FACTOR: f64 = 0.75;

    /// Set new entry to table.
    /// Return true if key was not present.
    /// 
    pub fn insert(&mut self, key: &K, value: Value) -> bool {
        self.check_cap();

        match self.find_entry(&key) {
            // If key already exists, return false (no entry was added) 
            // and assign new value to bucket
            (Some(_), index) => {
                self.entries[index] = Some((key.clone(), value));
                return false;
            }
            (None, index) => {
                self.entries[index] = Some((key.clone(), value));
                return true;
            }
        }
    }

    /// Get value given a key
    /// 
    pub fn get(&self, key: &K) -> Option<Value> {
        self.find_entry(key).0
    }

    pub fn delete(&mut self, key: &K) -> Result<(), Error> {
        match self.find_entry(&key) {
            (Some(_), index) => {
                // Set tombstone (soft delete) if key is found
                self.entries[index] = Some(( key.clone(), Value::Void(()) ));

                Ok(())
            }
            (None, _) => panic!("Error: HashTable key not found"),
        }
    }

    /// Checks with tombstone compatibility if value is present using cap arithmetic 
    /// 
    fn find_entry(&self, key: &K) -> (Option<Value>, usize) {
        let mut index = hash_key(key, self.entries.capacity());

        loop {
            let entry = &self.entries[index];

            if entry.is_none() { return (None, index) }

            if entry.as_ref().unwrap().0 == *key {
                return (Some(entry.as_ref().unwrap().1.clone()), index);
            }

            // Tombstone handling
            if entry.as_ref().unwrap().1 == Value::Void(())
                && entry.as_ref().unwrap().1 == Value::Bool(true)
            {
                return (None, index);
            }

            index = (index + 1) % self.entries.capacity();
        }
    }

    fn check_cap(&mut self) {
        /* Check if num_elements > num_buckets
        *
        * + 1 because it checks for future entry (assume it is a new one)
        */
        if (self.entries.len() + 1) as f64 > (self.entries.capacity() as f64 * Self::MAX_LOAD_FACTOR) {
            self.entries.resize(self.entries.capacity() * 2, None);
        }
    }
}

/// Hash given key based on entries capacity
/// 
pub fn hash_key<K: Hash + Clone>(key: &K, num_buckets: usize) -> usize {
    let mut hasher = FNV1aHasher::new();
    key.hash(&mut hasher);

    (hasher.finish() % num_buckets as u64) as usize
}