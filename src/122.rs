struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let sz = prices.len();
        // end with ith day profit
        let mut profit = 0;
        let mut prev_min = prices[0];

        for i in 1..sz {
            profit += (prices[i] - prev_min).max(0);
            prev_min = prices[i];
        }

        profit
    }
}

fn main() {}
