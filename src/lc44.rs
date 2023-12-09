struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ssz = s.len();
        let psz = p.len();
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let mut dp = vec![vec![false; psz + 1]; ssz + 1];
        dp[0][0] = true;

        for j in 1..=psz {
            if p[j - 1] == '*' {
                dp[0][j] = dp[0][j - 1];
            }
        }

        for i in 1..=ssz {
            for j in 1..=psz {
                if p[j - 1] == '?' || s[i - 1] == p[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[j - 1] == '*' {
                    // dont consider the * e.g. a -> a*
                    // consier the * case: e.g. ab -> a*
                    // that is dont consider the current char in s
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
            }
        }

        dp[ssz][psz]
    }
}
