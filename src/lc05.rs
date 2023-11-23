struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let sz = s.len();
        let mut dp = vec![vec![false; sz]; sz];
        for i in 0..sz {
            dp[i][i] = true;
        }

        let mut start = 0;
        let mut max_len = 1;

        for l in 2..=sz {
            for i in 0..=sz-l {
                let j = i + l - 1;
                dp[i][j] = s[i] == s[j] && (l <= 2 || dp[i+1][j - 1]);
                if dp[i][j] && l > max_len {
                    max_len = l;
                    start = i;
                }
            }
        }

        s[start..start+max_len].iter().collect()
    }
}
