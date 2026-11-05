struct Solution;
use std::collections::HashMap;


impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let cnt_j = *dp[j].get(&diff).unwrap_or(&0);
                let cnt_i = *dp[i].get(&diff).unwrap_or(&0);

                dp[i].insert(diff, cnt_i + cnt_j + 1);
                ret += cnt_j;
            }
        }

        ret
    }
}

fn main() {}
