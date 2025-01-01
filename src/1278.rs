#![allow(dead_code)]

struct Solution;

impl Solution {
    // 分成k份的最小change，让每一份都变成回文
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let k = k as usize;

        // cost[i][j] represents minimum changes needed to make s[i..=j] palindrome
        let mut cost = vec![vec![0; sz]; sz];
        for len in 2..=sz {
            for i in 0..=sz - len {
                let j = i + len - 1;
                cost[i][j] = cost[i + 1][j - 1] + if s[i] == s[j] { 0 } else { 1 };
            }
        }
        // dp[i][j] represents min changes needed to partition s[0..i] into j+1 groups
        let mut dp = vec![vec![i32::MAX / 2; k]; sz];
        for i in 0..sz {
            dp[i][0] = cost[0][i] as i32;
        }

        for j in 1..k {
            for i in j..sz {
                for l in j - 1..i {
                    dp[i][j] = dp[i][j].min(dp[l][j - 1] + cost[l + 1][i] as i32);
                }
            }
        }

        dp[sz - 1][k - 1]
    }
}

fn main() {}
