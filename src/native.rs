use std::time::Instant;

pub fn duration() {
    println!("{:?}", Instant::now().elapsed());
} 
