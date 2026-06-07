struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut dp = vec![0; sz + 1];
        for i in 1..=sz {
            // 1. take current
            let val1 = nums[i - 1] + if i > 1 { dp[i - 2] } else { 0 };
            let val2 = dp[i - 1];
            dp[i] = val1.max(val2);
        }

        dp[sz]
    }
}

fn main() {}
