#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let m = a.len();
        let n = b.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut ret = 0;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                dp[i][j] = if a[i] == b[j] {
                    1 + dp[i + 1][j + 1]
                } else {
                    0
                };
                ret = ret.max(dp[i][j]);
            }
        }

        ret
    }
}

fn main() {}
