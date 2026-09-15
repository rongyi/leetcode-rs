struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut ret = 2;

        // dp[i][d] end index is i, and diff is d
        let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); sz];

        for i in 0..sz {
            for j in 0..i {
                let d = nums[i] - nums[j];
                let prev = *(dp[j].get(&d).unwrap_or(&1));
                // chain together with this i
                let cur = prev + 1;
                let e = dp[i].entry(d).or_default();
                *e = (*e).max(cur);
                ret = ret.max(cur);
            }
        }

        ret
    }
}

fn main() {}
