use std::hash::Hasher;

const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

pub struct FNV1aHasher {
    hash: u64
}

impl FNV1aHasher {
    pub fn new() -> Self {
        FNV1aHasher { hash: OFFSET_BASIS }
    }
}

impl Hasher for FNV1aHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= byte as u64;
            self.hash = self.hash.wrapping_mul(FNV_PRIME);
        }
    }
}