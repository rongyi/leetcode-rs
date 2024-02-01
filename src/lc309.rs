struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let sz = prices.len();
        if sz < 2 {
            return 0;
        }
        // end day i when status is buy/sell/cooldown
        let mut buy = vec![0; sz];
        let mut sell = vec![0; sz];
        let mut cooldown = vec![0; sz];

        buy[0] = -prices[0];

        for i in 1..sz {
            // either do nothing or buy at day i, means cooldown at i - 1
            buy[i] = buy[i - 1].max(cooldown[i - 1] - prices[i]);
            // either do nothing or sell at day i, already minus the buy price
            // so here, we add sell price to get a profit
            sell[i] = sell[i - 1].max(buy[i - 1] + prices[i]);
            cooldown[i] = cooldown[i - 1].max(sell[i - 1]);
        }

        sell[sz - 1].max(cooldown[sz - 1])
    }
}
fn main() {}
