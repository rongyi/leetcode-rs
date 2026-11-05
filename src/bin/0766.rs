struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        // Check each diagonal starting from the first row
        for j in 0..n {
            let val = matrix[0][j];
            let mut i = 1;
            let mut k = j + 1;
            while i < m && k < n {
                if matrix[i][k] != val {
                    return false;
                }
                i += 1;
                k += 1;
            }
        }

        // Check each diagonal starting from the first column
        for i in 1..m {
            let val = matrix[i][0];
            let mut j = 1;
            let mut k = i + 1;
            while k < m && j < n {
                if matrix[k][j] != val {
                    return false;
                }
                j += 1;
                k += 1;
            }
        }

        true
    }
}

fn main() {}
