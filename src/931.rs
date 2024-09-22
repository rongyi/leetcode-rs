struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let sz = matrix.len();
        let mut dp = matrix.clone();
        for i in 1..sz {
            for j in 0..sz {
                let mut min_above = dp[i - 1][j];
                if j > 0 {
                    min_above = min_above.min(dp[i - 1][j - 1]);
                }
                if j < sz - 1 {
                    min_above = min_above.min(dp[i - 1][j + 1]);
                }
                dp[i][j] += min_above;
            }
        }

        *dp[sz - 1].iter().min().unwrap()
    }
}

fn main() {}
