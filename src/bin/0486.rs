struct Solution;

use std::cmp::max;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        // dp[i][j] represents the maximum net score a player can get
        // from the subarray nums[i..=j]
        let mut dp = vec![vec![0; n]; n];

        // Base case: Subarrays of length 1.
        // If there's only one number left, the player must take it.
        for i in 0..n {
            dp[i][i] = nums[i];
        }

        // Fill the table for subarray lengths from 2 up to n
        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;

                dp[i][j] = max(
                    nums[i] - dp[i + 1][j], // Option 1: Pick left element
                    nums[j] - dp[i][j - 1], // Option 2: Pick right element
                );
            }
        }

        // If the net score difference for the entire array is >= 0,
        // Player 1 can win or tie.
        dp[0][n - 1] >= 0
    }
}
fn main() {}
