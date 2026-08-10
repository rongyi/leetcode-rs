struct Solution;

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let m = a.len();
        let n = b.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                    ret = ret.max(dp[i + 1][j + 1]);
                }
            }
        }

        ret
    }
}

fn main() {}
