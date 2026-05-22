struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let m = w1.len();
        let n = w2.len();
        // dp[i][j] -> len i of w1 and lenj of w2
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        // init first row, first col
        // when other side is empty, we need to delete all nonempty str to make two same
        for i in 0..m {
            dp[i + 1][0] = (i + 1) as i32;
        }
        for j in 0..n {
            dp[0][j + 1] = (j + 1) as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                if w1[i - 1] == w2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    let v1 = dp[i - 1][j].min(dp[i][j - 1]) + 1;
                    let v2 = dp[i - 1][j - 1] + 1;
                    dp[i][j] = v1.min(v2);
                }
            }
        }

        dp[m][n]
    }
}

fn main() {}
