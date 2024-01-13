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
        let mut dp = vec![vec![0; sz]; k + 1];

        // Easy to understand explanation for the above solution

        // dp[i][j] = maximum profit from at most i transactions using prices[0..j]

        // A transaction is defined as one buy + sell.

        // Now on day j, we have two options

        // Do nothing (or buy) which doesn't change the acquired profit : dp[i][j] =
        // dp[i][j-1]

        // Sell the stock: In order to sell the stock, you must've bought it on a
        // day t=[0..j-1]. Maximum profit that can be attained is t:0->j-1
        // max(prices[j]-prices[t]+dp[i-1][t-1]) where prices[j]-prices[t] is the
        // profit from buying on day t and selling on day j. dp[i-1][t-1] is the
        // maximum profit that can be made with at most i-1 transactions with prices
        // prices[0..t-1].
        for t in 1..=k {
            let mut round_max_profit = -prices[0];
            for j in 1..sz {
                dp[t][j] = dp[t][j - 1].max(prices[j] + round_max_profit);
                round_max_profit = round_max_profit.max(dp[t - 1][j - 1] - prices[j]);
            }
        }

        dp[k][sz - 1]
    }
}

fn main() {}
