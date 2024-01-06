struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        // bottom up
        let mut dp = vec![0; n];
        let mut is_parlindrome = vec![vec![false; n]; n];

        for j in 0..n {
            // j + 1 number can have (j + 1 - 1) cut
            let mut min_cut = j;
            for i in 0..=j {
                if s[i] == s[j] && ((j - i) <= 1 || is_parlindrome[i + 1][j - 1]) {
                    is_parlindrome[i][j] = true;

                    if i == 0 {
                        min_cut = 0;
                    } else {
                        min_cut = min_cut.min(dp[i - 1] + 1);
                    }
                }
            }
            dp[j] = min_cut;
        }

        dp[n - 1] as i32
    }
}

fn main() {}
