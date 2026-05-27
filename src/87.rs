struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let sz = s1.len();
        let mut dp = vec![vec![vec![false; sz + 1]; sz]; sz];

        for i in 0..sz {
            for j in 0..sz {
                dp[i][j][1] = s1[i] == s2[j];
            }
        }
        for l in 2..=sz {
            for i in 0..=sz - l {
                for j in 0..=sz - l {
                    for k in 1..l {
                        // match from header
                        dp[i][j][l] |= dp[i][j][k] && dp[i + k][j + k][l - k];
                        // match from tail
                        dp[i][j][l] |= dp[i][j + l - k][k] && dp[i + k][j][l - k];
                    }
                }
            }
        }

        dp[0][0][sz]
    }
}

fn main() {}
