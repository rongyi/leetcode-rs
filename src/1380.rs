#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_min_idx = Vec::new();
        for i in 0..m {
            let mut min_idx = 0;
            let mut min_val = i32::MAX;
            for j in 0..n {
                if matrix[i][j] < min_val {
                    min_idx = j;
                    min_val = matrix[i][j];
                }
            }
            // for those column we check
            row_min_idx.push((min_idx, min_val));
        }
        let mut ret = Vec::new();
        for &(j, val) in row_min_idx.iter() {
            let mut max_col = i32::MIN;
            for i in 0..m {
                max_col = max_col.max(matrix[i][j]);
            }
            if max_col == val {
                ret.push(val);
            }
        }

        ret
    }
}

fn main() {}
