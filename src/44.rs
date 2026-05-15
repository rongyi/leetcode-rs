struct Solution;
struct SolutionTLE;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let m = s.len();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        // init '*' case
        for j in 1..=n {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 1];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    // the hard part:
                    // 1. dp[i][j - 1], just assume '*' not exist, and see if this is matched
                    // 2. dp[i - 1][j] just assume previous char not exist, swallowed by this '*' and check if it is match
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                } else if s[i - 1] == p[j - 1] || p[j - 1] == b'?' {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[m][n]
    }
}

impl SolutionTLE {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        Self::try_match(s, p)
    }

    fn try_match(s: &[u8], p: &[u8]) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }

        match p[0] {
            b'?' => {
                if s.is_empty() {
                    return false;
                }
                // simply ignore current char
                Self::try_match(&s[1..], &p[1..])
            }
            b'*' => {
                // Matches any sequence of characters (including the empty sequence)
                for i in 0..=s.len() {
                    if Self::try_match(&s[i..], &p[1..]) {
                        return true;
                    }
                }
                false
            }
            c @ _ => {
                if s.is_empty() {
                    return false;
                }
                // exact match
                if s[0] != c {
                    return false;
                }
                Self::try_match(&s[1..], &p[1..])
            }
        }
    }
}

fn main() {}
