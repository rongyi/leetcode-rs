struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    dp[i + 1][j + 1] = vec![dp[i][j + 1], dp[i + 1][j], dp[i][j]]
                        .into_iter()
                        .min()
                        .unwrap()
                        + 1;
                    ret = ret.max(dp[i + 1][j + 1]);
                }
            }
        }

        ret * ret
    }
}

fn main() {}
