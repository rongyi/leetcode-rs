#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut s0 = 0;
        let mut s1 = i32::MIN;
        // 系列总结:
        // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/108870/Most-consistent-ways-of-dealing-with-the-series-of-stock-problems
        for p in prices.into_iter() {
            // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/108867/C%2B%2B-concise-solution-O(n)-time-O(1)-space
            // s0 = profit having no stock
            // s1 = profit having 1 stock
            // update s0 by selling the stock from s1, so s0 = max(s0, s1+p);
            // update s1 by buying the stock from s0, so s1 = max(s1, s0-p-fee);

            let tmp = s0;
            s0 = s0.max(s1 + p);
            s1 = s1.max(tmp - p - fee);
        }
        s0
    }
}

fn main() {}
