struct Solution;

use std::i32;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // three state:
        // 1. hold, hold a stock
        // 2. sold sold today
        // 3. reset, not hold, not sold, do nothing today
        let mut hold = -prices[0];
        // impossible
        let mut sold = i32::MIN;
        // reset
        let mut reset = 0;

        for &p in prices.iter().skip(1) {
            let prev_hold = hold;
            let prev_sold = sold;
            let prev_reset = reset;

            // either prev hold or buy today
            // this is way better to understand
            hold = prev_hold.max(prev_reset - p);
            sold = prev_hold + p;
            reset = prev_reset.max(prev_sold);
        }

        sold.max(reset)
    }
}

fn main() {}
