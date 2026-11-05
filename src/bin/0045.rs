struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let sz = nums.len();

        let mut dp = vec![i32::MAX; sz];
        dp[0] = 0;

        let mut max_reach = (nums[0] as usize).min(sz - 1);
        let mut prev_reach = max_reach;
        let mut start = 0;
        loop {
            for i in start + 1..=max_reach {
                dp[i] = dp[i].min(dp[start] + 1);
                max_reach = max_reach.max((i + nums[i] as usize).min(sz - 1));
            }

            if max_reach == prev_reach {
                break;
            }
            start = prev_reach;
            prev_reach = max_reach;
        }

        dp[sz - 1]
    }
}

fn main() {}
