#![allow(dead_code)]

const DEFAULT_BUCKET_SIZE: usize = 10000;

struct MyHashMap {
    bucket: Vec<Vec<(i32, i32)>>,
    sz: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            bucket: vec![vec![]; DEFAULT_BUCKET_SIZE],
            sz: DEFAULT_BUCKET_SIZE,
        }
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % self.sz
    }

    fn put(&mut self, key: i32, value: i32) {
        let idx = self.hash(key);
        let mut exist = false;
        for (k, v) in self.bucket[idx].iter_mut() {
            if *k == key {
                exist = true;
                *v = value;
                break;
            }
        }
        if !exist {
            self.bucket[idx].push((key, value));
        }
    }

    fn get(&self, key: i32) -> i32 {
        let idx = self.hash(key);
        for (k, v) in self.bucket[idx].iter() {
            if *k == key {
                return *v;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let idx = self.hash(key);
        self.bucket[idx].retain(|(k, _v)| *k != key)
    }
}

fn main() {}
