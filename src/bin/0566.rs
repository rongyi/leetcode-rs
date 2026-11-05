struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let mut ret = vec![vec![0; c]; r];
        let m = mat.len();
        let n = mat[0].len();
        if r * c != m * n {
            return mat;
        }
        for i in 0..m {
            for j in 0..n {
                let idx = i * n + j;
                ret[idx / c][idx % c] = mat[i][j];
            }
        }

        ret
    }
}

fn main() {}
