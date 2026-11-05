struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        if amount == 0 {
            return 1;
        }
        let n = coins.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; amount + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            // 用前面i个硬币凑到和为j
            // 前面i个硬币的最后一个来说，可取可不取
            // 1. 不取，那么前面 i - 1 个硬币凑出和为 j 有多少种方法，那么 dp[i][j] 就有多少种
            //    也即此时dp[i][j] == dp[i - 1][j]
            // 2. 取了，那么dp[i][j - coints[i - 1]](amount为减去最后一个元素)包含i在内和为 j - coins[i - 1]
            //    有多少种方法
            dp[i][0] = 1;
            for j in 1..=amount {
                // don't take current
                dp[i][j as usize] = dp[i - 1][j as usize];
                // can and take current
                if j >= coins[i - 1] as usize {
                    dp[i][j] += dp[i][j - coins[i - 1] as usize]
                }
            }
        }

        dp[n][amount as usize]
    }
}

fn main() {}
