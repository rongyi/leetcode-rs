#![allow(dead_code)]
struct Solution;

impl Solution {
    // n count of numbers
    // m max num
    // k search cost
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        // dp[i][j][k]
        // array of length i, max value is j, search cost is k
        let n = n as usize;
        let m = m as usize;
        let cost = k as usize;
        let mut dp = vec![vec![vec![0; cost + 1]; m + 1]; n + 1];
        const MOD: i64 = 1e9 as i64 + 7;

        for j in 1..=m {
            dp[1][j][1] = 1;
        }

        for i in 1..=n {
            for j in 1..=m {
                for k in 1..=cost {
                    let mut sum: i64 = 0;
                    //最大值不变所以查询代价不变，[1..j] inclusive
                    //的所有元素都可以append过来
                    sum = (sum + j as i64 * dp[i - 1][j][k]) % MOD;

                    for x in 1..j {
                        sum = (sum + dp[i - 1][x][k - 1]) % MOD;
                    }
                    dp[i][j][k] = (dp[i][j][k] + sum) % MOD;
                }
            }
        }
        let mut ret: i64 = 0;
        for j in 1..=m {
            ret = (ret + dp[n][j][cost]) % MOD;
        }

        ret as i32
    }
}

fn main() {}
