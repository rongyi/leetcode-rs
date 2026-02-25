struct Solution;

impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        // dp[i] stores the max length of a subsequence that sums to i
        // Initialize with -1 to represent "unreachable"
        let mut dp = vec![-1; target + 1];
        dp[0] = 0;

        for num in nums {
            let n = num as usize;
            // Iterate backwards to ensure each number is used only once (0/1 Knapsack)
            for s in (n..=target).rev() {
                // If the previous sum (s - n) was reachable
                if dp[s - n] != -1 {
                    dp[s] = dp[s].max(dp[s - n] + 1);
                }
            }
        }

        dp[target]
    }
}

fn main() {}
