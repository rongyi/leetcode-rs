struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut cnt = HashMap::new();
        for card in deck.into_iter() {
            *cnt.entry(card).or_insert(0) += 1;
        }

        let mut val = 0;
        for &c in cnt.values() {
            val = Self::gcd(val, c);
        }

        val > 1
    }

    // b is the smaller one
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {}
