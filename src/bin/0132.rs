struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let mut is_parlindrome = vec![vec![false; sz]; sz];

        let mut dp = vec![0; sz];
        for j in 0..sz {
            // j + 1 total nums have j + 1 - 1 cut
            let mut min_cut = j;

            // check
            for i in 0..=j {
                if s[i] == s[j] && (j - i <= 1 || is_parlindrome[i + 1][j - 1]) {
                    is_parlindrome[i][j] = true;
                    if i == 0 {
                        min_cut = 0;
                        // dont break, to fill parlindrome table
                    } else {
                        min_cut = min_cut.min(dp[i - 1] + 1);
                    }
                }
            }

            dp[j] = min_cut;
        }

        dp[sz - 1] as _
    }
}

fn main() {
    Solution::min_cut("aab".to_string());
}
