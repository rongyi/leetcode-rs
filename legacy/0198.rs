struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut dp = vec![0; sz];
        for i in 0..sz {
            // 1. take current
            let sum1 = nums[i] + if i > 1 { dp[i - 2] } else { 0 };
            // 2. ignore current
            let sum2 = if i > 0 { dp[i - 1] } else { 0 };
            dp[i] = sum1.max(sum2);
        }

        dp[sz - 1]
    }
}

fn main() {}
