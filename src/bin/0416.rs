struct Solution;

impl Solution {
    /// LeetCode 416: Partition Equal Subset Sum
    ///
    /// 2D DP: dp[i][j] = can the first i numbers sum to j?
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }

        let half = (sum / 2) as usize;
        let n = nums.len();

        // dp[i][j]: using first i numbers, can we reach sum j?
        // find the reverse order hard to understand
        let mut dp = vec![vec![false; half + 1]; n + 1];
        dp[0][0] = true; // empty subset sums to 0

        for i in 0..n {
            for j in 0..=half {
                dp[i + 1][j] = dp[i][j] || (j >= nums[i] as usize && dp[i][j - nums[i] as usize]);
            }
        }

        dp[n][half]
    }
}

fn main() {}
