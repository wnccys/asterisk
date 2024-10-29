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
        if ((self.count + 1 / self.entries.capacity()) as f32) > Self::MAX_LOAD {
            self.entries.reserve((self.count as f32 / Self::MAX_LOAD).ceil() as usize);
        }

        if let Some(new_entry) = self.find_entry(&key) {
            self.entries.fill(Some(new_entry));
            return true;
        }

        return false;
    }

    fn find_entry(&self, key: &Vec<char>) -> Option<Rc<Entry>> {
        let mut index = hash_string(key) as usize % self.entries.capacity();

        loop {
            let entry = self.entries[index].as_ref().unwrap();

            if entry.key == *key || self.entries[index].is_none() {
                return Some(Rc::clone(entry));
            }

            index = (index + 1) % self.entries.capacity();
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all_entries: Vec<(Vec<char>, Value)> = Default::default(); 
        let mut final_formated: String = Default::default();

        self.entries.iter().for_each(|entry| {
            all_entries.push((entry.as_ref().unwrap().key.clone(), entry.as_ref().unwrap().value.clone()));
        });

        all_entries.iter().for_each(|key_value| {
            final_formated += format!("{:?} => {:?}", key_value.0, key_value.1).as_str();
        });

        write!(f, "{final_formated}")
    }
}

pub fn hash_string(key: &Vec<char>) -> Hash {
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

    #[test]
    fn test_find_entry_by_key() {
        let mut table = Table::default();
        let str = vec!['l', 'o', 'l', 'o',' ', 'm', 'e', 'u', ' ', 'a', 'm', 'o', 'r'];
        table.set(vec!['a', 'm', 'o', 'r'], Value::String(str));
        println!("{table}");
    }
}
