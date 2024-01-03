struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut buy1 = i32::MIN;
        let mut sell1 = 0;
        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for p in prices {
            sell2 = sell2.max(buy2 + p);
            buy2 = buy2.max(sell1 - p);
            sell1 = sell1.max(buy1 + p);
            buy1 = buy1.max(-p);
        }

        sell2
    }
}

fn main() {}
