use std::{fmt::Error, rc::Rc};

use crate::value::Value;
pub type Hash = u32;

struct Entry {
    key: Vec<char>,
    value: Value,
}

pub struct Table {
    count: usize,
    entries: Box<Vec<Option<Rc<Entry>>>>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            count: 0,
            entries: Box::new(Vec::with_capacity(4)),
        }
    }
}

impl Table {
    const MAX_LOAD: f32 = 0.75;

    fn set(&mut self, key: Vec<char>, value: Value) -> bool {
        if ((self.count / self.entries.capacity()) as f32) > Self::MAX_LOAD {
            self.entries.reserve((self.count as f32 / Self::MAX_LOAD).ceil() as usize);
        }

        if let Some(new_entry) = self.find_entry(&mut key.to_owned()) {
            self.entries.fill(Some(new_entry));
            return true;
        }

        return false;
    }

    fn find_entry(&self, key: &mut Vec<char>) -> Option<Rc<Entry>> {
        let mut index = hash_string(key) as usize % self.entries.capacity();

        loop {
            let entry = self.entries[index].as_ref().unwrap();

            // match entry.key {}

            if entry.key == *key || self.entries[index].is_none() {
                return Some(Rc::clone(entry));
            }

            index = (index + 1) % self.entries.capacity();
        }
    }
}

pub fn hash_string(key: &mut Vec<char>) -> Hash {
    let mut hash: Hash = 2166136261;

    key.iter().for_each(|&c| {
        if let Some(digit) = c.to_digit(36) {
            hash ^= digit;
            hash = hash.wrapping_mul(16777619);
        }
    });

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        let mut word = vec!['n', 'u', 'm'];
        println!("hash: {}", hash_string(&mut word));

        let mut word = vec!['n', 'a', 'm'];
        println!("hash: {}", hash_string(&mut word));

        let mut word = vec!['n', 'u', 'l', 'l'];
        println!("hash: {}", hash_string(&mut word));

        let mut word = vec!['z', 'a', 'z', 'a'];
        println!("hash: {}", hash_string(&mut word));
    }

    fn test_find_entry_by_key() {
        let table = Table::default();
        let str = vec!['l', 'o', 'l', 'o',' ', 'm', 'e', 'u', ' ', 'a', 'm', 'o', 'r'];
        table.set();
    }
}
