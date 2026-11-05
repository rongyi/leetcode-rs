struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut val_idx: HashMap<i32, usize> = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {
            if let Some(&j) = val_idx.get(&(target - val)) {
                return vec![i as i32, j as i32];
            }

            val_idx.insert(val, i);
        }

        unreachable!()
    }
}

fn main() {}
