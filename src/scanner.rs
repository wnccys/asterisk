pub struct Scanner<'a> {
    start: &'a str,
    current: &'a str,
    line: i32,
}