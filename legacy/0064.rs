struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = grid.clone();
        let m = dp.len();
        let n = dp[0].len();

        for i in 1..m {
            dp[i][0] += dp[i - 1][0];
        }
        for j in 1..n {
            dp[0][j] += dp[0][j - 1];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
            }
        }

        dp[m - 1][n - 1]
    }
}
