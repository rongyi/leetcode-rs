struct Solution;

impl Solution {
    //  If the two ends of a string are the same, then they must be included in
    //  the longest palindrome subsequence. Otherwise, both ends cannot be
    //  included in the longest palindrome subsequence.
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let m = s.len();
        let mut dp = vec![vec![0; m]; m];
        Self::recur(0, (s.len() - 1) as i32, &s, &mut dp)
    }

    fn recur(l: i32, r: i32, s: &[char], dp: &mut Vec<Vec<i32>>) -> i32 {
        if l == r {
            return 1;
        }
        if l > r {
            return 0;
        }
        let lu = l as usize;
        let ru = r as usize;
        if dp[lu][ru] != 0 {
            return dp[lu][ru];
        }
        if s[lu] == s[ru] {
            dp[lu][ru] = 2 + Self::recur(l + 1, r - 1, s, dp);
            return dp[lu][ru];
        }
        dp[lu][ru] = Self::recur(l, r - 1, s, dp).max(Self::recur(l + 1, r, s, dp));
        dp[lu][ru]
    }
}

fn main() {}
