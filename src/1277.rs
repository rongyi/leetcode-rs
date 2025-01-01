#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        // represents the size of the largest square submatrix ending at position
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    dp[i + 1][j + 1] = 1 + dp[i][j + 1].min(dp[i + 1][j].min(dp[i][j]));
                    ret += dp[i + 1][j + 1];
                }
            }
        }
        ret
    }
}

fn main() {}
