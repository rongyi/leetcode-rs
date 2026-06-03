struct Solution;

use std::collections::{HashMap, VecDeque};
struct LRUCache {
    // key
    lru_order: VecDeque<i32>,
    // key -> pos
    kv: HashMap<i32, i32>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            kv: HashMap::new(),
            lru_order: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.kv.contains_key(&key) {
            self.lru_order.retain(|&k| k != key);
            self.lru_order.push_back(key);

            *self.kv.get(&key).unwrap()
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.kv.contains_key(&key) {
            self.lru_order.retain(|&k| k != key);
            self.lru_order.push_back(key);
            self.kv.insert(key, value);
        } else {
            if self.kv.len() < self.capacity {
                self.lru_order.push_back(key);
                self.kv.insert(key, value);
            } else {
                self.kv.remove(&self.lru_order.pop_front().unwrap());
                self.lru_order.push_back(key);
                self.kv.insert(key, value);
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
fn main() {}
