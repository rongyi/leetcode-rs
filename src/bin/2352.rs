struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut rows: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut cols: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut ret = 0;
        for i in 0..n {
            *rows.entry(grid[i].clone()).or_default() += 1;
        }
        for j in 0..n {
            let vals: Vec<i32> = (0..n).map(|i| grid[i][j]).collect();
            *cols.entry(vals).or_default() += 1;
        }

        for (k, v) in rows.into_iter() {
            if let Some(&v2) = cols.get(&k) {
                ret += v * v2;
            }
        }

        ret
    }
}

fn main() {}
