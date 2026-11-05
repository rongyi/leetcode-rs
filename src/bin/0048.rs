struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        let mut r0 = 0;
        let mut r1 = n - 1;
        while r0 < r1 {
            for j in 0..n {
                let tmp = matrix[r0][j];
                matrix[r0][j] = matrix[r1][j];
                matrix[r1][j] = tmp;
            }
            r0 += 1;
            r1 -= 1;
        }

        for i in 0..n {
            for j in i..n {
                let v = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = v;
            }
        }
    }
}

fn main() {}
