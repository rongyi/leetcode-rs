struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ret = i32::MIN;
        let mut sum = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut tmp = matrix[i][j];
                if i > 0 {
                    tmp += sum[i - 1][j];
                }
                if j > 0 {
                    tmp += sum[i][j - 1];
                }
                // 多加的部分减回去
                if i > 0 && j > 0 {
                    tmp -= sum[i - 1][j - 1];
                }

                sum[i][j] = tmp;

                for r in 0..=i {
                    for c in 0..=j {
                        let mut tmp = sum[i][j];
                        if r > 0 {
                            tmp -= sum[r - 1][j];
                        }
                        if c > 0 {
                            tmp -= sum[i][c - 1];
                        }
                        // 多减的加回来
                        if r > 0 && c > 0 {
                            tmp += sum[r - 1][c - 1];
                        }
                        if tmp <= k {
                            ret = ret.max(tmp);
                        }
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
