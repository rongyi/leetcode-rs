struct Solution;

use std::{collections::HashMap, i32};
impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut cache_idx: HashMap<i32, usize> = HashMap::new();
        let mut ret = usize::MAX;
        for (i, &v) in cards.iter().enumerate() {
            if cache_idx.contains_key(&v) {
                let prev = *cache_idx.get(&v).unwrap();
                let distance = i - prev + 1;
                ret = ret.min(distance);
            }

            cache_idx.insert(v, i);
        }
        if ret == usize::MAX {
            return -1;
        }
        ret as _
    }
}

fn main() {}
