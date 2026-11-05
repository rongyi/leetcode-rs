struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums[0];
        }
        let sz = nums.len();

        Self::rob_line(&nums[0..sz - 1]).max(Self::rob_line(&nums[1..sz]))
    }

    fn rob_line(nums: &[i32]) -> i32 {
        let sz = nums.len();
        let mut dp = vec![0; sz];

        for i in 0..sz {
            // 1. take current
            let take = nums[i] + if i >= 2 { dp[i - 2] } else { 0 };
            // 2. ignore
            let ignore = if i > 0 { dp[i - 1] } else { 0 };
            dp[i] = take.max(ignore);
        }

        dp[sz - 1]
    }
}

fn main() {}
