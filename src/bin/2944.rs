struct Solution;

use std::i32;
impl Solution {
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let sz = prices.len();
        let mut dp = vec![0; sz + 2];

        for i in (1..=sz).rev() {
            if i + i >= sz {
                dp[i] = prices[i - 1];
            } else {
                let mut next_min = i32::MAX;
                for j in (i + 1)..=(2 * i + 1) {
                    next_min = next_min.min(dp[j]);
                }
                dp[i] = prices[i - 1] + next_min;
            }
        }
        dp[1]
    }
}

fn main() {}
