struct Solution;

use rand::Rng;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    cache: HashMap<i32, HashSet<usize>>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            nums: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.nums.push(val);
        let e = self.cache.entry(val).or_insert(HashSet::new());
        e.insert(self.nums.len() - 1);

        e.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(target_set) = self.cache.get_mut(&val) {
            if let Some(&deleted_idx) = target_set.iter().next() {
                target_set.remove(&deleted_idx);
                if target_set.is_empty() {
                    self.cache.remove(&val);
                }
                let last_idx = self.nums.len() - 1;
                let last_val = self.nums[last_idx];
                if deleted_idx != self.nums.len() - 1 {
                    let last_idx_set = self.cache.get_mut(&last_val).unwrap();
                    last_idx_set.remove(&(self.nums.len() - 1));
                    last_idx_set.insert(deleted_idx);
                    self.nums[deleted_idx] = last_val;
                }

                self.nums.pop();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let idx = rand::thread_rng().gen_range(0..self.nums.len());

        self.nums[idx]
    }
}

fn main() {}
