struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return false;
        }
        let mut win_sum = 0;
        for i in 0..2 {
            win_sum += nums[i];
        }
        let mut sum_cache: HashSet<i32> = HashSet::new();
        sum_cache.insert(win_sum);
        for i in 2..nums.len() {
            win_sum += nums[i];
            win_sum -= nums[i - 2];
            if sum_cache.contains(&win_sum) {
                return true;
            }
            sum_cache.insert(win_sum);
        }

        false
    }
}

fn main() {}
