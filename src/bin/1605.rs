#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let m = row_sum.len();
        let n = col_sum.len();
        let mut matrix = vec![vec![0; n]; m];
        let mut row_remaining = row_sum.clone();
        let mut col_remaining = col_sum.clone();

        for i in 0..m {
            for j in 0..n {
                let val = std::cmp::min(row_remaining[i], col_remaining[j]);
                matrix[i][j] = val;
                row_remaining[i] -= val;
                col_remaining[j] -= val;
            }
        }

        matrix
    }
}
fn main() {}
