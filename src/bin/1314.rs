#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut prefix = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                prefix[i + 1][j + 1] =
                    mat[i][j] + prefix[i + 1][j] + prefix[i][j + 1] - prefix[i][j];
            }
        }

        let mut ret = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let x1 = 0.max(i as i32 - k) as usize;
                let y1 = 0.max(j as i32 - k) as usize;

                let x2 = (m - 1).min(i + k as usize);
                let y2 = (n - 1).min(j + k as usize);

                let val = prefix[x2 + 1][y2 + 1] - prefix[x2 + 1][y1] - prefix[x1][y2 + 1]
                    + prefix[x1][y1];
                ret[i][j] = val;
            }
        }

        ret
    }
}

fn main() {}
