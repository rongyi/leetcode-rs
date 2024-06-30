#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                rows[i] = rows[i].max(grid[i][j]);
                cols[j] = cols[j].max(grid[i][j]);
            }
        }

        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                ret += (rows[i] - grid[i][j]).min(cols[j] - grid[i][j]);
            }
        }

        ret
    }
}

fn main() {}
