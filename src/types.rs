use std::{fmt::Error, rc::Rc};

use crate::value::Value;
pub type Hash = u32;

#[derive(Debug)]
pub struct Entry {
    pub key: Vec<char>,
    pub value: Value,
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
            entries: vec![None, None, None, None],
        }
    }
}

impl Table {
    const MAX_LOAD: f32 = 0.75;

    // TODO Handle tombstone ghost counting
    /// Set new entry to table.
    /// Return true if key was not present.
    /// 
    pub fn set(&mut self, key: &Vec<char>, value: Value) -> bool {
        self.check_cap();

        if key.len() == 0 {
            return false;
        };

        // Applied when some new entry is found or None is returned
        match self.find_entry(&key) {
            (Some(_), index) => {
                self.entries[index] = Some(Rc::new(Entry { key: key.to_owned(), value }));
                return false;
            }
            (None, index) => {
                self.entries[index] = Some(Rc::new(Entry { key: key.to_owned(), value }));
                self.count += 1;
                return true;
            }
        }
    }

    pub fn get(&self, key: &Vec<char>) -> Option<Rc<Entry>> {
        if self.count == 0 {
            return None;
        }
        self.find_entry(key).0
    }

    pub fn delete(&mut self, key: &Vec<char>) -> Result<(), Error> {
        if self.count == 0 {
            return Err(Error);
        };

        match self.find_entry(key) {
            (Some(_), index) => {
                // Unreachable key
                self.entries[index] = Some(Rc::new(Entry {
                    key: "".chars().collect(),
                    value: Value::Bool(true),
                }));
                Ok(())
            }
            (None, _) => Err(Error),
        }
    }

    /// Checks with tombstone compatibility if value is present using cap arithmetic 
    /// 
    fn find_entry(&self, key: &Vec<char>) -> (Option<Rc<Entry>>, usize) {
        let mut index = hash_string(key) as usize % self.entries.capacity();

        loop {
            let entry = self.entries[index].to_owned();

            if entry.is_none() || entry.as_ref().unwrap().key == *key {
                return (entry, index);
            }

            // Tombstone handling
            if entry.as_ref().unwrap().key == "".chars().collect::<Vec<char>>()
                && entry.as_ref().unwrap().value == Value::Bool(true)
            {
                return (None, index);
            }

            index = (index + 1) % self.entries.capacity();
        }
    }

    fn check_cap(&mut self) {
        if ((self.count + 1 / self.entries.capacity()) as f32) >= Self::MAX_LOAD {
            self.entries
                .resize(((self.count + 1) as f32 / Self::MAX_LOAD).ceil() as usize, None);
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

// TODO rewrite tests
// TODO set test to new/already set results on Table::set() return;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests if new entries returns false on set over existent entry or true for new entries.
    fn test_set_new_entry_bool() {

    }

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
        let str: Vec<char> = "meu amor".chars().collect();
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

        table.set(
            &"jesse".chars().collect(),
            Value::String("james".chars().collect()),
        );
        println!(
            "Result: {:?}",
            table.get(&"jesse".chars().collect()).unwrap()
        );
    }

    #[test]
    fn test_table_del() {
        let mut table = Table::default();

        table.set(
            &"name".chars().collect(),
            Value::String("JOJI".chars().collect()),
        );
        table.delete(&"name".chars().collect()).unwrap();
        assert_eq!(table.get(&"name".chars().collect()).is_none(), true);
    }
}
