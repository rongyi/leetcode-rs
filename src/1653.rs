#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let sz = s.len();
        let mut dp = vec![0; sz + 1];
        let mut bcount = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'b' {
                bcount += 1;
                dp[i + 1] = dp[i];
            } else {
                dp[i + 1] = (dp[i] + 1).min(bcount);
            }
        }

        dp[sz]
    }
}
fn main() {}
