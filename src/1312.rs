#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut dp = vec![vec![0; sz]; sz];
        let lonest_prefix = Self::dfs(&s, 0, sz - 1, &mut dp);
        sz as i32 - lonest_prefix
    }

    fn dfs(s: &Vec<char>, l: usize, r: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if l == r {
            return 1;
        }
        if l > r {
            return 0;
        }
        if dp[l][r] > 0 {
            return dp[l][r];
        }
        if s[l] == s[r] {
            dp[l][r] = 2 + Self::dfs(s, l + 1, r - 1, dp);
            return dp[l][r];
        }

        dp[l][r] = Self::dfs(s, l + 1, r, dp).max(Self::dfs(s, l, r - 1, dp));

        dp[l][r]
    }
}

fn main() {}
