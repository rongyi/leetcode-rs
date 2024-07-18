#![allow(dead_code)]
struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();

        for &h in hand.iter() {
            cnt.entry(h).and_modify(|v| *v += 1).or_insert(1);
        }

        while !cnt.is_empty() {
            let &cur = cnt.keys().next().unwrap();
            for w in 0..group_size {
                let val = cnt.get(&(cur + w));
                if val.is_none() {
                    return false;
                }
                cnt.entry(cur + w).and_modify(|v| *v -= 1);
                if *cnt.get(&(cur + w)).unwrap() == 0 {
                    cnt.remove(&(cur + w));
                }
            }
        }

        true
    }
}

fn main() {}
