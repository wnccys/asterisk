use std::hash::Hasher;

pub struct FNV1aHasher {
    hash: u64,
}

impl FNV1aHasher {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    pub fn new() -> Self {
        FNV1aHasher {
            hash: Self::OFFSET_BASIS,
        }
    }
}

impl Hasher for FNV1aHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= byte as u64;
            self.hash = self.hash.wrapping_mul(Self::FNV_PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}
