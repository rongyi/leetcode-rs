#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let m = 1e9 as i32 + 7;
        // dp[i][j] i rolling sum to j
        let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
        dp[0][0] = 1;

        // rolling dice round
        for i in 1..=n {
            for j in 1..=target {
                // target too big to reach, each rolling get max value k, and still not reach this value
                if j > i * k {
                    break;
                }
                for l in 1..=k.min(j) {
                    dp[i as usize][j as usize] =
                        (dp[i as usize][j as usize] + dp[(i - 1) as usize][(j - l) as usize]) % m;
                }
            }
        }

        dp[n as usize][target as usize]
    }
}

fn main() {}
