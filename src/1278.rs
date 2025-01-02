#![allow(dead_code)]

struct Solution;

impl Solution {
    // 分成k份的最小change，让每一份都变成回文
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let k = k as usize;

        // cost[i][j] represents minimum changes needed to make s[i..=j] palindrome
        let mut cost = vec![vec![0; sz]; sz];
        for len in 2..=sz {
            for i in 0..=sz - len {
                let j = i + len - 1;
                cost[i][j] = cost[i + 1][j - 1] + if s[i] == s[j] { 0 } else { 1 };
            }
        }
        // dp[i][j] end to =j and split to i chunk
        let mut dp = vec![vec![i32::MAX / 2; sz]; k + 1];
        // Initialize for k=1
        for i in 0..sz {
            // 分成一份的值就是从0 ==> i的子字符创需要改变的值
            dp[1][i] = cost[0][i];
        }

        for k in 2..=k {
            for i in k - 1..sz {
                for j in ((k - 2)..=(i - 1)).rev() {
                    // i 是终点， j是中间劈开的点, j往左移， i慢慢放开
                    // j劈开的左边部分是 dp[k - 1][j], 右边是change[j + 1][i]
                    dp[k][i] = dp[k][i].min(cost[j + 1][i] + dp[k - 1][j]);
                }
            }
        }

        dp[k][sz - 1]
    }
}

fn main() {}
