#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut all_cuts = cuts.clone();
        all_cuts.push(0);
        all_cuts.push(n);
        all_cuts.sort_unstable();

        let m = all_cuts.len();
        let mut dp = vec![vec![0; m]; m];

        // This DP solution solves the min cost to cut a stick problem.
        // dp[i][j] represents the minimum cost to cut the stick from position all_cuts[i] to all_cuts[j].
        //
        // We first add 0 and n to our cuts array and sort it to represent all potential cutting positions.
        // Then we populate our DP table:
        // - For each length (difference between two positions)
        // - For each starting position i
        // - We try every possible middle cut k and choose the minimum cost
        // - The cost includes: cost of cutting left segment + cost of cutting right segment + cost of current cut
        //   (which is the length of the segment we're cutting: all_cuts[j] - all_cuts[i])
        //
        // We return dp[0][m-1] which is the min cost to cut the entire stick from 0 to n.
        for len in 2..m {
            for i in 0..m - len {
                let j = i + len;
                dp[i][j] = i32::MAX;

                for k in i + 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + all_cuts[j] - all_cuts[i]);
                }
            }
        }

        dp[0][m - 1]
    }
}

fn main() {}
