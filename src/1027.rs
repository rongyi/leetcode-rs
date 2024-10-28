struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        // dp[diff][index]
        let mut dp: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let mut ret = 2;
        let sz = nums.len();
        for i in 0..sz {
            for j in i + 1..sz {
                let d = nums[j] - nums[i];
                if dp
                    .entry(d)
                    .or_insert(HashMap::new())
                    .contains_key(&(i as i32))
                {
                    let val = dp[&d][&(i as i32)];
                    dp.entry(d).and_modify(|c| {
                        c.insert(j as i32, val + 1);
                    });
                } else {
                    dp.entry(d).and_modify(|c| {
                        c.insert(j as i32, 2);
                    });
                }
                ret = ret.max(dp[&d][&(j as i32)]);
            }
        }

        ret
    }
}

fn main() {}
