#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stone_value[i];
        }

        // dp[i][j] represents the maximum score Alice can get from stones[i...j]
        let mut dp = vec![vec![0; n]; n];

        // Length of subarray
        for len in 2..=n {
            // Starting index i
            for i in 0..=n - len {
                let j = i + len - 1; // Ending index

                // Try all possible partition points
                for k in i..j {
                    let left_sum = prefix_sum[k + 1] - prefix_sum[i];
                    let right_sum = prefix_sum[j + 1] - prefix_sum[k + 1];

                    if left_sum < right_sum {
                        dp[i][j] = dp[i][j].max(left_sum + dp[i][k]);
                    } else if left_sum > right_sum {
                        dp[i][j] = dp[i][j].max(right_sum + dp[k + 1][j]);
                    } else {
                        dp[i][j] = dp[i][j].max(left_sum + dp[i][k].max(dp[k + 1][j]));
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}

fn main() {}
