#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut len = 0;

        // prefix sum
        for i in 0..m {
            for j in 0..n {
                dp[i + 1][j + 1] = mat[i][j] + dp[i][j + 1] + dp[i + 1][j] - dp[i][j];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                let r = i.min(j);
                for k in 1..=r {
                    let val = dp[i][j] - dp[i - k][j] - dp[i][j - k] + dp[i - k][j - k];
                    if val <= threshold {
                        len = len.max(k);
                    }
                }
            }
        }

        len as i32
    }
}

fn main() {}
