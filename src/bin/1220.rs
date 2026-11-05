#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let n = n as usize;
        let m = 1e9 as i64 + 7;
        let mut dp = vec![vec![0; 5]; n + 1];
        for i in 0..5 {
            dp[1][i] = 1;
        }
        // 0 1 2 3 4
        // a e i o u
        for i in 1..n {
            // a can be reached by e i u
            dp[i + 1][0] = (dp[i][1] + dp[i][2] + dp[i][4]) % m;
            // e <== a i
            dp[i + 1][1] = (dp[i][0] + dp[i][2]) % m;
            // i <== e o
            dp[i + 1][2] = (dp[i][1] + dp[i][3]) % m;
            // o <== i
            dp[i + 1][3] = dp[i][2];
            // u <== i o
            dp[i + 1][4] = (dp[i][2] + dp[i][3]) % m;
        }
        let mut ret = 0;

        for i in 0..5 {
            ret = (ret + dp[n][i]) % m;
        }

        ret as i32
    }
}

fn main() {}
