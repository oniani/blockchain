use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Block {
    pub data: Vec<u8>,
    pub hash: u64,
    pub nonce: usize,
    pub previous_hash: Option<u64>,
}

impl Block {
    pub fn new(data: Vec<u8>, hash: u64, nonce: usize, previous_hash: Option<u64>) -> Self {
        Block {
            data,
            hash,
            nonce,
            previous_hash,
        }
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (&self.data, self.nonce).hash(&mut hasher);
        hasher.finish()
    }

    pub fn mine(&mut self, difficulty: u32) {
        self.hash = self.hash();
        // NOTE: DefaultHasher produces 64-bit hashes.
        while self.hash > u64::pow(2, 64 - difficulty) {
            self.nonce += 1;
            self.hash = self.hash();
        }
    }
}
