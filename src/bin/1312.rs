#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let mut dp = vec![vec![0; sz]; sz];

        for len in 2..=sz {
            for i in 0..=sz - len {
                let j = i + len - 1;
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    // insert at left or right
                    // acc -> insert at right
                    // aac -> insert at left
                    dp[i][j] = dp[i + 1][j].min(dp[i][j - 1]) + 1;
                }
            }
        }

        dp[0][sz - 1]
    }
}

fn main() {}
