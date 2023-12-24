struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        let m = s1.len();
        let n = s2.len();
        if m + n != s3.len() {
            return false;
        }
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        // only use s1
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] && s1[i - 1] == s3[i - 1];
        }
        // only use s2
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] && s2[j - 1] == s3[j - 1];
        }
        for i in 1..=m {
            for j in 1..=n {
                let k = i + j - 1;
                dp[i][j] =
                    (dp[i - 1][j] && s1[i - 1] == s3[k]) || (dp[i][j - 1] && s2[j - 1] == s3[k]);
            }
        }

        dp[m][n]
    }
}

