#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut result = 0;

        // For each row, calculate consecutive ones to the left
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    dp[i][j] = if j > 0 { dp[i][j - 1] + 1 } else { 1 };

                    // For each possible height starting from the current position
                    let mut width = dp[i][j];
                    for k in (0..=i).rev() {
                        width = width.min(dp[k][j]);
                        result += width;
                        if width == 0 {
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}

fn main() {}
