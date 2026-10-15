#![allow(dead_code)]
struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
            const MOD: i64 = 1_000_000_007;
            // memo[dice_left][rem_target] initialized to -1
            let mut memo = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];

            Self::dfs(n, target, k, &mut memo) as i32
        }

        fn dfs(dice: i32, rem_target: i32, k: i32, memo: &mut Vec<Vec<i64>>) -> i64 {
            const MOD: i64 = 1_000_000_007;

            // Base cases
            if dice == 0 {
                return if rem_target == 0 { 1 } else { 0 };
            }
            if rem_target < 0 {
                return 0;
            }

            let d = dice as usize;
            let t = rem_target as usize;

            // Return cached result if available
            if memo[d][t] != -1 {
                return memo[d][t];
            }

            let mut ways = 0;
            for face in 1..=k {
                ways = (ways + Self::dfs(dice - 1, rem_target - face, k, memo)) % MOD;
            }

            // Cache and return
            memo[d][t] = ways;
            ways
        }
    }
}

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
