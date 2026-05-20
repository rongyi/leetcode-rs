struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            dp[i + 1][1] = dp[i][1] + grid[i][0];
        }
        for j in 0..n {
            dp[1][j + 1] = dp[1][j] + grid[0][j];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i + 1][j + 1] = grid[i][j] + dp[i][j + 1].min(dp[i + 1][j]);
            }
        }

        dp[m][n]
    }
}

fn main() {}
