struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        // obstacle 1
        let m = grid.len();
        let n = grid[0].len();
        // never go to target
        if grid[m - 1][n - 1] == 1 || grid[0][0] == 1 {
            return 0;
        }
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            if grid[i][0] == 1 {
                break;
            } else {
                dp[i][0] = 1;
            }
        }
        for j in 0..n {
            if grid[0][j] == 1 {
                break;
            } else {
                dp[0][j] = 1;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if grid[i - 1][j] != 1 {
                    dp[i][j] += dp[i - 1][j];
                }
                if grid[i][j - 1] != 1 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

fn main() {}
