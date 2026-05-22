struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut first_row_has_zero = false;
        let mut first_col_has_zero = false;
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col_has_zero = true;
                break;
            }
        }
        for j in 0..n {
            if matrix[0][j] == 0 {
                first_row_has_zero = true;
                break;
            }
        }

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }
        for i in 1..m {
            if matrix[i][0] == 0 {
                for v in matrix[i].iter_mut() {
                    *v = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }
        if first_row_has_zero {
            for v in matrix[0].iter_mut() {
                *v = 0;
            }
        }
        if first_col_has_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

fn main() {}
