#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];

        // Calculate prefix sums for quick range sums
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        // Create a 2D DP array where dp[i][j] represents the maximum score difference
        // for the player who moves first on the subarray stones[i..=j]
        let mut dp = vec![vec![0; n]; n];

        // dp[i][j] = max(
        //      sum(i+1..=j) - dp[i+1][j],  // remove stones[i]
        //      sum(i..=j-1) - dp[i][j-1]   // remove stones[j]
        // )

        // Fill the dp table diagonally
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;

                // Sum of stones[i+1..=j]
                let sum_i_plus_1_to_j = prefix_sum[j + 1] - prefix_sum[i + 1];

                // Sum of stones[i..=j-1]
                let sum_i_to_j_minus_1 = prefix_sum[j] - prefix_sum[i];

                // Calculate max score difference
                dp[i][j] =
                    (sum_i_plus_1_to_j - dp[i + 1][j]).max(sum_i_to_j_minus_1 - dp[i][j - 1]);
            }
        }

        dp[0][n - 1]
    }
}

fn main() {}
