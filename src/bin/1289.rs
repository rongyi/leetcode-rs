#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        for i in (0..n - 1).rev() {
            for j in 0..n {
                let mut val = i32::MAX;
                for k in 0..j {
                    val = val.min(grid[i + 1][k]);
                }
                for k in j + 1..n {
                    val = val.min(grid[i + 1][k]);
                }
                grid[i][j] += val;
            }
        }

        *grid[0].iter().min().unwrap()
    }
}

fn main() {}
