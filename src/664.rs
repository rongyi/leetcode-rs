struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut uniq = 1;
        let mut s: Vec<char> = s.chars().collect();
        for i in 1..s.len() {
            if s[i] != s[(uniq - 1) as usize] {
                s[uniq as usize] = s[i];
                uniq += 1;
            }
        }
        s.resize(uniq as usize, '0');
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            for j in i..n {
                dp[i][j] = if i == j { 1 } else { dp[i + 1][j] + 1 };
                for k in (i + 1)..=j {
                    if s[k] == s[i] {
                        dp[i][j] = dp[i][j].min(dp[i + 1][k - 1] + dp[k][j]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}

fn main() {
    Solution::strange_printer("aaabbb".to_string());
}
