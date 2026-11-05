struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; 51]; 50001];
        Self::dfs(&grid, 0, 0, k, &mut dp, 0)
    }
    fn dfs(
        grid: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        k: i32,
        dp: &mut Vec<Vec<i32>>,
        cur: i32,
    ) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if i >= m || j >= n {
            return 0;
        }
        if i == m - 1 && j == n - 1 {
            return if (cur + grid[i][j] % k) == 0 { 1 } else { 0 };
        }
        if dp[i * n + j][cur as usize] == 0 {
            dp[i * n + j][cur as usize] = (1
                + Self::dfs(grid, i + 1, j, k, dp, (cur + grid[i][j]) % k)
                + Self::dfs(grid, i, j + 1, k, dp, (cur + grid[i][j] % k)))
                % 1_000_000_007;
        }
        dp[i * n + j][cur as usize] - 1
    }
}

fn main() {}
