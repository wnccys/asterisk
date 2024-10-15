use crate::value::Value;

struct Entry {
    key: Vec<char>,
    value: Value
}

pub struct Table<'a> {
    count: i64,
    capacity: i64,
    entries: &'a Entry,
}