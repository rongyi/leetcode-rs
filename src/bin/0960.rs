struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();

        let m = strs.len();
        let n = strs[0].len();
        let mut ret = (n - 1) as i32;
        let mut dp = vec![1; n];

        for j in 0..n {
            for i in 0..j {
                let mut k = 0;
                while k < m {
                    // not valid
                    if strs[k][i] > strs[k][j] {
                        break;
                    }
                    k += 1;
                }
                if k == m && dp[i] + 1 > dp[j] {
                    dp[j] = dp[i] + 1;
                }
            }
            ret = ret.min(n as i32 - dp[j]);
        }

        ret
    }
}

fn main() {}
