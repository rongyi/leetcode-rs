struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();

        for &h in hand.iter() {
            *cnt.entry(h).or_default() += 1;
        }

        while !cnt.is_empty() {
            let &cur = cnt.keys().next().unwrap();
            for cur_step in 0..group_size {
                if let Some(val) = cnt.get_mut(&(cur + cur_step)) {
                    *val -= 1;
                    if *val == 0 {
                        cnt.remove(&(cur + cur_step));
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}
