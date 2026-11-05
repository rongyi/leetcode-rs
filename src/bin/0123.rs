struct Solution;

use std::i32;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;
        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        // think buy as negative profit
        for &p in prices.iter() {
            buy1 = buy1.max(-p);
            sell1 = sell1.max(buy1 + p);
            buy2 = buy2.max(sell1 - p);
            sell2 = sell2.max(buy2 + p);
        }

        sell2
    }
}

fn main() {}
