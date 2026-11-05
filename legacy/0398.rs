use std::collections::HashMap;

use rand::thread_rng;
use rand::Rng;

struct Solution {
    cache: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut cache = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let e = cache.entry(num).or_insert(Vec::new());
            e.push(i as i32)
        }

        Self { cache }
    }

    fn pick(&self, target: i32) -> i32 {
        if let Some(idx_lst) = self.cache.get(&target) {
            if idx_lst.len() == 1 {
                return idx_lst[0];
            }
            let mut rng = thread_rng();
            let idx = rng.gen_range(0..idx_lst.len());
            idx_lst[idx]
        } else {
            -1
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

fn main() {}
