struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prev_low = prices[0];
        let mut max_profit = 0;

        for today_price in prices.into_iter().skip(1) {
            let cur_profit = today_price - prev_low;
            max_profit = max_profit.max(cur_profit);

            prev_low = prev_low.min(today_price);
        }

        max_profit
    }
}

fn main() {}
