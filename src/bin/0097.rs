struct SolutionTLE;

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let m = s1.len();
        let n = s2.len();
        if m + n != s3.len() {
            return false;
        }
        // dp[i][j] use s1 len i, s2 len j to form s3: i + j
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        // use only of s1
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] && s1[i - 1] == s3[i - 1];
        }
        // use only of s2
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] && s2[j - 1] == s3[j - 1];
        }

        for i in 1..=m {
            for j in 1..=n {
                let k = i + j;
                dp[i][j] = dp[i - 1][j] && s1[i - 1] == s3[k - 1]
                    || dp[i][j - 1] && s2[j - 1] == s3[k - 1];
            }
        }

        dp[m][n]
    }
}

impl SolutionTLE {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        Self::try_interleave(s1, 0, s2, 0, s3, 0, true)
            || Self::try_interleave(s1, 0, s2, 0, s3, 0, false)
    }
    fn try_interleave(
        s1: &[u8],
        pos1: usize,
        s2: &[u8],
        pos2: usize,
        s3: &[u8],
        pos3: usize,
        s1_turn: bool,
    ) -> bool {
        if pos3 == s3.len() && pos1 == s1.len() && pos2 == s2.len() {
            return true;
        }
        if pos3 == s3.len() {
            return false;
        }
        if s1_turn {
            let mut total_match = 0;
            while pos1 + total_match < s1.len()
                && pos3 + total_match < s3.len()
                && s1[pos1 + total_match] == s3[pos3 + total_match]
            {
                total_match += 1;
            }
            // no even one char
            if total_match == 0 {
                return false;
            }
            for l in 1..=total_match {
                if Self::try_interleave(s1, pos1 + l, s2, pos2, s3, pos3 + l, !s1_turn) {
                    return true;
                }
            }

            false
        } else {
            let mut total_match = 0;
            while pos2 + total_match < s2.len()
                && pos3 + total_match < s3.len()
                && s2[pos2 + total_match] == s3[pos3 + total_match]
            {
                total_match += 1;
            }
            // no even one char
            if total_match == 0 {
                return false;
            }
            for l in 1..=total_match {
                if Self::try_interleave(s1, pos1, s2, pos2 + l, s3, pos3 + l, !s1_turn) {
                    return true;
                }
            }

            false
        }
    }
}

fn main() {
    let s1 = "".to_string();
    let s2 = "abc".to_string();
    let s3 = "ab".to_string();
    Solution::is_interleave(s1, s2, s3);
}
