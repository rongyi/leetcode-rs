#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        // dp[i][j][k] 第i次扔，最后那个数字是j，并且已经连续出现k次了
        let n = n as usize;
        let mut dp = vec![vec![vec![0; 16]; 6]; n];
        for i in 0..6 {
            dp[0][i][1] = 1;
        }
        for i in 1..n {
            for j in 0..6 {
                for k in 0..6 {
                    if k == j {
                        continue;
                    }
                    for m in 1..=roll_max[k] as usize {
                        dp[i][j][1] = (dp[i][j][1] + dp[i - 1][k][m]) % MOD;
                    }
                }
                for m in 2..=roll_max[j] as usize {
                    dp[i][j][m] = dp[i - 1][j][m - 1];
                }
            }
        }
        let mut ret = 0;
        for i in 0..6 {
            for j in 1..=roll_max[i] as usize {
                ret = (ret + dp[n - 1][i][j]) % MOD;
            }
        }

        ret
    }
}
fn main() {}
