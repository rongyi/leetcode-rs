#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let sz = m * n;
        let k = k as usize % sz;

        if k == 0 {
            return grid;
        }

        let mut vals: Vec<i32> = Vec::with_capacity(sz);
        for i in 0..m {
            for j in 0..n {
                vals.push(grid[i][j]);
            }
        }
        vals.extend(vals.clone());
        let mut idx = 0;
        let mut j = sz - k;
        while idx < sz {
            grid[idx / n][idx % n] = vals[j];
            idx += 1;
            j += 1;
        }

        grid
    }
}

fn main() {}
