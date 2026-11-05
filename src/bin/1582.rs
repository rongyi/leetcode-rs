#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        // Count 1s in each row and column
        let mut row_count = vec![0; m];
        let mut col_count = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                }
            }
        }

        // Count special positions
        let mut special_count = 0;

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 && row_count[i] == 1 && col_count[j] == 1 {
                    special_count += 1;
                }
            }
        }

        special_count
    }
}

fn main() {}
