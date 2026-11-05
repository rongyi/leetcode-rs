struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prev_min = prices[0];
        let mut max_profit = None;

        for i in 1..prices.len() {
            if prices[i] - prev_min > 0 {
                max_profit = max_profit.max(Some(prices[i] - prev_min));
            }
            prev_min = prev_min.min(prices[i]);
        }
        match max_profit {
            Some(val) => val,
            None => 0,
        }
    }
}

fn main() {}
