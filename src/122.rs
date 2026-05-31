struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut acc_profit = 0;

        for (i, &p) in prices.iter().enumerate() {
            if i > 0 {
                acc_profit += (p - prices[i - 1]).max(0);
            }
        }

        acc_profit
    }
}

fn main() {}
