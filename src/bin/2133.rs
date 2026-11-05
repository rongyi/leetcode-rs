struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let expected_set: HashSet<i32> = (1..=n as i32).collect();
        for i in 0..n {
            let cur_row: HashSet<i32> = matrix[i].iter().copied().collect();
            if !cur_row.eq(&expected_set) {
                return false;
            }
        }

        for j in 0..n {
            let mut cur_col = HashSet::new();
            for i in 0..n {
                cur_col.insert(matrix[i][j]);
            }
            if !cur_col.eq(&expected_set) {
                return false;
            }
        }

        true
    }
}

fn main() {}
