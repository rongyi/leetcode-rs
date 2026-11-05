
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut valid: HashMap<i32, i64> = HashMap::new();
        let mut acc = 0;
        for (i, num) in nums.into_iter().enumerate() {
            let k = num - i as i32;
            let prev_valid = *valid.get(&k).unwrap_or(&0);
            acc += i as i64 - prev_valid;

            *valid.entry(k).or_default() += 1;
        }

        acc
    }
}

fn main() {}
