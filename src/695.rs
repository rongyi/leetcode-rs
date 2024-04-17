#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let mut cur_max = 0;
                    Self::dfs(&mut grid, i as i32, j as i32, &mut cur_max);
                    ret = ret.max(cur_max);
                }
            }
        }

        ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, cur_max: &mut i32) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] == 0 {
            return;
        }

        grid[i as usize][j as usize] = 0;
        *cur_max += 1;

        Self::dfs(grid, i + 1, j, cur_max);
        Self::dfs(grid, i - 1, j, cur_max);
        Self::dfs(grid, i, j + 1, cur_max);
        Self::dfs(grid, i, j - 1, cur_max);
    }
}

fn main() {}
