struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ret = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                ret[i][j] = matrix[j][i];
            }
        }

        ret
    }
}

fn main() {}
