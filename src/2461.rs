struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(|v| v as i64).collect();
        let mut win_nums: HashMap<i64, i32> = HashMap::new();
        let k = k as usize;
        let mut wing_acc = 0;
        for i in 0..k {
            *win_nums.entry(nums[i]).or_default() += 1;
            wing_acc += nums[i];
        }
        let mut max_sum = 0;
        if win_nums.len() == k {
            max_sum = max_sum.max(wing_acc);
        }
        for i in k..nums.len() {
            *win_nums.entry(nums[i]).or_default() += 1;
            *win_nums.entry(nums[i - k]).or_default() -= 1;
            if win_nums[&nums[i - k]] == 0 {
                win_nums.remove(&nums[i - k]);
            }
            wing_acc += nums[i];
            wing_acc -= nums[i - k];
            if win_nums.len() == k {
                max_sum = max_sum.max(wing_acc);
            }
        }

        max_sum
    }
}

fn main() {}
