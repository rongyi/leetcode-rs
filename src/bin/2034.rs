
pub struct Solution;

use std::collections::BTreeMap;
struct StockPrice {
    // all prices
    prices: BTreeMap<i32, i32>,
    // ts -> price
    candle: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        Self {
            candle: BTreeMap::new(),
            prices: BTreeMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if self.candle.contains_key(&timestamp) {
            let old_price = self.candle[&timestamp];
            self.prices.entry(old_price).and_modify(|v| *v -= 1);
            if self.prices[&old_price] == 0 {
                self.prices.remove(&old_price);
            }
        }

        self.candle.insert(timestamp, price);
        *self.prices.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        let last = self.candle.iter().rev().next().unwrap();
        *last.1
    }

    fn maximum(&self) -> i32 {
        *self.prices.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.prices.iter().next().unwrap().0
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */

fn main() {}
