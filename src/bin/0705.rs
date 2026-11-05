#![allow(dead_code)]

const INITIAL_KEY_SPACE: usize = 10000;

struct MyHashSet {
    bucket: Vec<Vec<i32>>,
    sz: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            bucket: vec![vec![]; INITIAL_KEY_SPACE],
            sz: INITIAL_KEY_SPACE,
        }
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % INITIAL_KEY_SPACE
    }

    fn add(&mut self, key: i32) {
        let idx = self.hash(key);
        self.bucket[idx].push(key);
    }

    fn remove(&mut self, key: i32) {
        let idx = self.hash(key);
        self.bucket[idx].retain(|&x| x != key)
    }

    fn contains(&self, key: i32) -> bool {
        let idx = self.hash(key);
        self.bucket[idx].contains(&key)
    }
}

fn main() {}
