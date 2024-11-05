use std::{fmt::Error, rc::Rc};

use crate::value::Value;
pub type Hash = u32;

#[derive(Debug)]
struct Entry {
    key: Vec<char>,
    value: Value,
}

#[derive(Debug)]
pub struct Table {
    count: usize,
    entries: Vec<Option<Rc<Entry>>>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            count: 0,
            entries: vec![None],
        }
    }
}

impl Table {
    const MAX_LOAD: f32 = 0.75;

    pub fn set(&mut self, key: &Vec<char>, value: Value) -> bool {
        self.check_cap();
        let key = key.clone();

        // applied when some new entry is found or None is returned
        match self.find_entry(&key) {
            (Some(new_entry), index) => {
                 self.entries[index] = Some(new_entry);
                 return true;
            },
            (None, index) => { 
                self.entries[index] = Some(Rc::new(Entry{ key, value }));
                self.count += 1;
                return true;
            },
        }
    }

    pub fn get(&self, key: &Vec<char>) -> Option<Rc<Entry>> {
        if self.count == 0 { return None }
        self.find_entry(key).0
    }
     
    pub fn delete() {}

    fn find_entry(&self, key: &Vec<char>) -> (Option<Rc<Entry>>, usize) {
        let mut index = hash_string(key) as usize % self.entries.capacity();

        loop {
            let entry = self.entries[index].to_owned();

            if self.entries[index].is_none() || entry.as_ref().unwrap().key == *key {
                return (entry, index);
            }

            index = (index + 1) % self.entries.capacity();
        }
    }
    
    fn check_cap(&mut self) {
        if ((self.count + 1 / self.entries.capacity()) as f32) > Self::MAX_LOAD {
            self.entries.reserve((self.count as f32 / Self::MAX_LOAD).ceil() as usize);
        }
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

        let mut word = vec!['n', 'u', 'm'];
        println!("hash: {}", hash_string(&mut word));

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
        let str: Vec<char> = "lolo meu amor".chars().collect();
        println!("cap before: {}", table.entries.capacity());
        println!("count before add element: {}", table.count);
        table.set(&"amor".chars().collect(), Value::String(str.clone()));
        table.set(&"amor".chars().collect(), Value::String(str.clone()));
        let key: Vec<char> = "amor".chars().collect();
        println!("found: {:?}", table.find_entry(&key));
        println!("{:?}", table);
        println!("count after add element: {}", table.count);
        println!("cap after: {}", table.entries.capacity());
    }

    #[test]
    fn test_table_get() {
        let mut table = Table::default();

        table.set(&"jesse".chars().collect(), Value::String("james".chars().collect()));
        println!("Result: {:?}", table.get(&"jesse".chars().collect()).unwrap());
    }
}
