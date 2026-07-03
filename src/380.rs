/// LeetCode 380: Insert Delete GetRandom O(1)
///
/// HashMap + Vec with swap-remove. LeetCode provides `rand`.
use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    nums: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            nums: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.nums.len());
        self.nums.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        let Some(&pos) = self.map.get(&val) else {
            return false;
        };
        let last_idx = self.nums.len() - 1;
        let last_val = self.nums[last_idx];

        if pos != last_idx {
            self.nums.swap(pos, last_idx);
            self.map.insert(last_val, pos);
        }
        self.nums.pop();
        self.map.remove(&val);
        true
    }

    fn get_random(&self) -> i32 {
        let idx = rand::thread_rng().gen_range(0..self.nums.len());
        self.nums[idx]
    }
}

fn main() {}
