impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let m = s.len();
        let n = t.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        // ways to match empty string subsequece is 1
        for i in 0..=m {
            dp[i][0] = 1;
        }

        for i in 1..=m {
            for j in 1..=n {
                if s[i - 1] == t[j - 1] {
                    // we can use this last char in s or not
                    // if we use we can get dp[i- 1][j - 1]
                    // if we dont use, dp[i - 1][j] is the number
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        dp[m][n]
    }
}

