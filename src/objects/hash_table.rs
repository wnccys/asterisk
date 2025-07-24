use crate::utils::hasher::FNV1aHasher;
use std::{
    cell::RefCell,
    fmt::Display,
    hash::{Hash, Hasher},
    rc::Rc,
};

pub type Entry<K, V> = (K, Rc<RefCell<V>>);

#[derive(Debug, Clone)]
pub struct HashTable<K, V> {
    pub entries: Vec<Option<(K, Rc<RefCell<V>>)>>,
}

impl<K: Clone, V: Clone> Default for HashTable<K, V> {
    fn default() -> Self {
        Self {
            entries: vec![None; 4],
        }
    }
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Clone + PartialEq + Display + std::fmt::Debug,
    V: Default + Clone + std::fmt::Debug 
{
    const MAX_LOAD_FACTOR: f64 = 0.75;

    /// Set new entry to table.
    /// Return true if key is new.
    ///
    pub fn insert(&mut self, key: &K, value: V) -> bool {
        self.check_cap();

        let entry = self.find_mut(&key);
        let is_new = entry.is_none();

        // Create new bucket with associated Rc if new; Otherside internally mut already set RefCell.
        if is_new {
            *entry = Some((key.clone(), Rc::new(RefCell::new(value))));
        } else {
            *entry.as_ref().unwrap().1.borrow_mut() = value;
        }


        /* If key already exists, return false (no entry was added) and assign new value to bucket */
        return is_new;
    }

    /// Get value given a key
    ///
    pub fn get(&self, key: &K) -> Option<Rc<RefCell<V>>> {
        self.find(key)
    }

    pub fn delete(&mut self, key: &K) -> bool {
        let entry = self.find_mut(key);

        if entry.is_none() {
            return false;
        }

        /* Take already set key and a tombstone value */
        *entry.as_ref().unwrap().1.borrow_mut() = V::default();

        true
    }

    /// Checks with tombstone compatibility if value is present using cap arithmetic
    ///
    fn find(&self, key: &K) -> Option<Rc<RefCell<V>>> {
        let current_cap = self.entries.capacity();
        let mut index = hash_key(key, self.entries.capacity());

        loop {
            if self.entries[index].is_none() {
                return None;
            }

            /* Compare found entry key with given key */
            if self.entries.get(index).unwrap().as_ref().unwrap().0 == *key 
            {
                let (_, val_ref) = &self.entries[index].as_ref().unwrap();
                return Some(Rc::clone(val_ref));
            }

            /* TODO Add tombstone handling */

            index = (index + 1) % current_cap;
        }
    }

    /// Checks with tombstone compatibility if value is present using cap arithmetic
    ///
    fn find_mut(&mut self, key: &K) -> &mut Option<Entry<K, V>> {
        let current_cap = self.entries.capacity();
        let mut index = hash_key(key, self.entries.capacity());

        loop {
            if self.entries[index].is_none() { return &mut self.entries[index]; }

            /* Compare found entry key with given key */
            if self.entries[index].as_ref().unwrap().0 == *key {
                return &mut self.entries[index];
            }

            /* TODO Add tombstone handling */

            index = (index + 1) % current_cap;
        }
    }

    fn check_cap(&mut self) {
        /* Check if num_elements (Some) > num_buckets
         *
         * + 1 because it checks for future entry (assume it is a new one)
         */
        if (self.entries.iter().filter(|e| e.is_some()).count() + 1) as f64
            > (self.entries.capacity() as f64 * Self::MAX_LOAD_FACTOR)
        {
            self.resize();
        }
    }

    /// Custom resize implementation because all entries needs to be re-hashed after resize for proper late hash recover
    ///
    fn resize(&mut self) {
        let new_buck_count = self.entries.capacity() * 2;
        let mut new_entries: Vec<Option<Entry<K,V>>> = vec![None; new_buck_count];

        for bucket in self.entries.drain(..) {
            if let Some((k, v)) = bucket {
                /* Target index  */
                let mut t_idx = hash_key(&k, new_buck_count);
                let _target = &new_entries[t_idx];

                if _target.is_some() {
                    t_idx  = Self::probe_idx(&new_entries, t_idx);
                }

                new_entries[t_idx] = Some((k, v));
            }
        }

        self.entries = new_entries;
    }

    pub fn probe_idx(new_entries: &Vec<Option<Entry<K, V>>>, init_idx: usize) -> usize {
        let cap = new_entries.capacity();
        /* Set bounds for idx handling based on current cap */
        let mut probe_idx = (init_idx + 1) % cap;

        // This operation never fails, as the array objectivelly has new empty space with the resize
        for bucket in new_entries[probe_idx..].iter().cycle() {
            if bucket.is_none() { return probe_idx }

            probe_idx = (probe_idx + 1) % cap;
        }

        /* Dummy panic */
        panic!();
    }
}

/// Hash given key based on entries capacity
///
pub fn hash_key<K: Hash + Clone + Display>(key: &K, num_buckets: usize) -> usize {
    let mut hasher = FNV1aHasher::new();
    key.hash(&mut hasher);

    (hasher.finish() % num_buckets as u64) as usize
}