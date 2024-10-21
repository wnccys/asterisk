use std::rc::Rc;

use crate::value::Value;
pub type Hash = u32;


struct Entry {
    key: Vec<char>,
    value: Value
}

pub struct Table {
    count: i64,
    entries: Option<Vec<Rc<Entry>>>,
}

impl Default for Table {
    fn default() -> Self {
       Self {
            count: 0,
            entries: None
       } 
    }
}

impl Table {
    fn set(&mut self, key: Vec<char>, value: Value) -> bool {
        true
    }
}

pub fn hash_string(key: &mut Vec<char>) -> Hash {
    let mut hash: Hash = 2166136261;

    for i in 0..key.len() {
        println!("i is {i}, key is: {}", key[i]);
        hash ^= key[i].to_digit(36).unwrap();
        hash = hash.wrapping_mul(16777619);
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        let mut word = vec!{'n', 'u', 'm'};
        println!("hash: {}", hash_string(&mut word));

        let mut word = vec!{'n', 'a', 'm'};
        println!("hash: {}", hash_string(&mut word));

        let mut word = vec!{'n', 'u', 'l', 'l'};
        println!("hash: {}", hash_string(&mut word));

        let mut word = vec!{'z', 'a', 'z', 'a'};
        println!("hash: {}", hash_string(&mut word));
    }
}