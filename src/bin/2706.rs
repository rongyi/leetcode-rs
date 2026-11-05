struct Solution;

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        let sum = prices[0] + prices[1];
        if sum <= money {
            return money - sum;
        }
        money
    }
}

fn main() {}
