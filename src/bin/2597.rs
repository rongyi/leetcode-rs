struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut cur_set = HashMap::new();

        Self::dfs(&nums, 0, &mut cur_set, k) - 1
    }

    fn dfs(nums: &[i32], idx: usize, cur_set: &mut HashMap<i32, i32>, k: i32) -> i32 {
        if idx == nums.len() {
            return 1;
        }
        let mut ret = 0;
        // 1. dont take current
        ret += Self::dfs(nums, idx + 1, cur_set, k);
        // 2. take current, but with condition check
        if *cur_set.get(&(nums[idx] + k)).unwrap_or(&0) <= 0
            && *cur_set.get(&(nums[idx] - k)).unwrap_or(&0) <= 0
        {
            *cur_set.entry(nums[idx]).or_default() += 1;
            ret += Self::dfs(nums, idx + 1, cur_set, k);
            *cur_set.entry(nums[idx]).or_default() -= 1;
        }

        ret
    }
}

fn main() {}
