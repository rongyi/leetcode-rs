#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    let cur_sum = Self::dfs(&mut grid, i as i32, j as i32);
                    ret = ret.max(cur_sum);
                }
            }
        }

        ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        // out of grid, or 0
        if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] == 0 {
            return 0;
        }

        let acc = grid[i as usize][j as usize];
        grid[i as usize][j as usize] = 0;

        // four direction and can not go back, each direction is a subpath
        let s1 = Self::dfs(grid, i + 1, j);
        let s2 = Self::dfs(grid, i - 1, j);
        let s3 = Self::dfs(grid, i, j + 1);
        let s4 = Self::dfs(grid, i, j - 1);

        grid[i as usize][j as usize] = acc;

        acc + s1.max(s2.max(s3.max(s4)))
    }
}

fn main() {}
