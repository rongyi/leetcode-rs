struct Solution;

use std::collections::{HashMap, VecDeque};
struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    lru_order: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::with_capacity(capacity as usize),
            lru_order: VecDeque::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            self.lru_order.retain(|&k| k != key);
            self.lru_order.push_back(key);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.lru_order.retain(|&k| k != key);
        } else if self.cache.len() >= self.capacity {
            self.cache.remove(&self.lru_order.pop_front().unwrap());
        }
        self.cache.insert(key, value);
        self.lru_order.push_back(key);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {}
