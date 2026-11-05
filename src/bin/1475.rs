#![allow(dead_code)]


struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut vals: BTreeSet<(usize, i32)> = BTreeSet::new();
        let mut ret = prices.clone();
        for i in (0..prices.len()).rev() {
            let cur_price = prices[i];
            let mut discount = 0;
            for &(_idx, after_price) in vals.iter() {
                if after_price <= cur_price {
                    discount = after_price;
                    break;
                }
            }
            ret[i] = cur_price - discount;

            vals.insert((i, prices[i]));
        }

        ret
    }
}

fn main() {}
