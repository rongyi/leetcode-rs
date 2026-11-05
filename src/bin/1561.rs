#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let sz = piles.len() / 3;
        let mut ret = 0;

        // After sorting, we need to:
        // - Give the n smallest piles to Bob
        // - We take the second-largest pile of each remaining triplet
        // - Alice takes the largest pile of each triplet
        for i in (sz..piles.len()).step_by(2) {
            ret += piles[i];
        }

        ret
    }
}

fn main() {}
