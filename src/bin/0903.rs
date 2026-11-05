struct Solution;

impl Solution {
    // https://leetcode.com/problems/valid-permutations-for-di-sequence/solutions/196939/easy-to-understand-solution-with-detailed-explanation/
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let m = 1e9 as i32 + 7;
        // dp[i][j] represents the number of valid permutations for the
        // first i characters of the sequence, ending with the number j.
        let mut dp = vec![vec![0; sz + 1]; sz + 1];
        for j in 0..=sz {
            dp[0][j] = 1;
        }

        for i in 1..=sz {
            for j in 0..=i {
                if s[i - 1] == 'D' {
                    for k in j..i {
                        dp[i][j] = dp[i][j] % m + dp[i - 1][k] % m;
                    }
                } else {
                    for k in 0..j {
                        dp[i][j] = dp[i][j] % m + dp[i - 1][k] % m;
                    }
                }
            }
        }
        let mut ret = 0;
        for j in 0..=sz {
            ret = ret % m + dp[sz][j] % m;
        }

        ret % m
    }
}

fn main() {}
