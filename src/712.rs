#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let m = s1.len();
        let n = s2.len();
        // dp[i][j] is the cost for s1.substr(0,i) and s2.substr(0, j). Note s1[i],
        // s2[j] not included in the substring.

        // Base case: dp[0][0] = 0
        // target: dp[m][n]

        // if s1[i-1] = s2[j-1]   // no deletion
        //     dp[i][j] = dp[i-1][j-1];
        // else   // delete either s1[i-1] or s2[j-1]
        //     dp[i][j] = min(dp[i-1][j]+s1[i-1], dp[i][j-1]+s2[j-1]);
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for j in 1..=n {
            // delete all s2
            dp[0][j] = dp[0][j - 1] + s2[j - 1] as i32;
        }

        for i in 1..=m {
            // delete all s1 till i - 1
            dp[i][0] = dp[i - 1][0] + s1[i - 1] as i32;
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    // just follow
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    // delete one of theme
                    dp[i][j] =
                        (dp[i - 1][j] + s1[i - 1] as i32).min(dp[i][j - 1] + s2[j - 1] as i32);
                }
            }
        }

        dp[m][n]
    }
}

fn main() {}
