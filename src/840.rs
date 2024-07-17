#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m < 3 || n < 3 {
            return 0;
        }
        let mut ret = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                if Self::valid(&grid, i, j) {
                    ret += 1;
                }
            }
        }

        ret
    }

    fn valid(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
        if grid[x + 1][y + 1] != 5 {
            return false;
        }
        let mut uniq = HashSet::new();
        for i in 0..3 {
            for j in 0..3 {
                uniq.insert(grid[x + i][y + j]);
            }
        }
        if uniq.len() != 9 {
            return false;
        }
        for i in 1..=9 {
            uniq.remove(&i);
        }
        if !uniq.is_empty() {
            return false;
        }

        // sum check
        for i in 0..3 {
            if grid[x + i][y] + grid[x + i][y + 1] + grid[x + i][y + 2] != 15 {
                return false;
            }
        }
        for j in 0..3 {
            if grid[x][y + j] + grid[x + 1][y + j] + grid[x + 2][y + j] != 15 {
                return false;
            }
        }
        if grid[x][y] + grid[x + 1][y + 1] + grid[x + 2][y + 2] != 15 {
            return false;
        }
        if grid[x][y + 2] + grid[x + 1][y + 1] + grid[x + 2][y] != 15 {
            return false;
        }

        true
    }
}

fn main() {}
