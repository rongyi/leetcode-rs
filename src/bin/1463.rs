#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        // dp[row][robote1_col][robot2_col]
        let mut dp = vec![vec![vec![-1; n]; n]; m];
        Self::dfs(&grid, &mut dp, m, n, 0, 0, n - 1)
    }

    fn dfs(
        grid: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<Vec<i32>>>,
        m: usize,
        n: usize,
        cur_row: usize,
        r1_col: usize,
        r2_col: usize,
    ) -> i32 {
        if cur_row == m {
            return 0;
        }
        if dp[cur_row][r1_col][r2_col] != -1 {
            return dp[cur_row][r1_col][r2_col];
        }
        let mut ret = 0;
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                let r1_next_col = r1_col as i32 + i;
                let r2_next_col = r2_col as i32 + j;
                // out of range
                if r1_next_col < 0
                    || r1_next_col >= n as i32
                    || r2_next_col < 0
                    || r2_next_col >= n as i32
                {
                    continue;
                }
                let next_level = Self::dfs(
                    grid,
                    dp,
                    m,
                    n,
                    cur_row + 1,
                    r1_next_col as usize,
                    r2_next_col as usize,
                );
                ret = ret.max(next_level);
            }
        }
        let cur_cherry = if r1_col == r2_col {
            grid[cur_row][r1_col]
        } else {
            grid[cur_row][r1_col] + grid[cur_row][r2_col]
        };
        dp[cur_row][r1_col][r2_col] = ret + cur_cherry;
        dp[cur_row][r1_col][r2_col]
    }
}

fn main() {}
