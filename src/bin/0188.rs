struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let sz = prices.len();
        let k = k as usize;
        if k >= sz / 2 {
            let mut sum = 0;
            for i in 1..sz {
                sum += (prices[i] - prices[i - 1]).max(0);
            }
            return sum;
        }
        // dp[i][j] = max profit with at most j transactions up to day i
        let mut dp = vec![vec![0; k + 1]; sz];

        for j in 1..=k {
            let mut max_diff = -prices[0];
            for i in 1..sz {
                // 1. donothing, that means finish j tx before i -> dp[i - 1][j], or just make a tx, that is sell at current price
                dp[i][j] = dp[i - 1][j].max(prices[i] + max_diff);
                max_diff = max_diff.max(dp[i - 1][j - 1] - prices[i]);
            }
        }

        dp[sz - 1][k]
    }
}

fn main() {}
