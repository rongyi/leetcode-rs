#![allow(dead_code)]


struct Solution;

use std::collections::HashSet;
impl Solution {
    // means the digit facing down not showen in any cards in facing up
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let sz = fronts.len();
        let mut same: HashSet<i32> = HashSet::new();
        for i in 0..sz {
            if fronts[i] == backs[i] {
                same.insert(fronts[i]);
            }
        }
        let mut ret = 3000;
        for &num in fronts.iter() {
            if !same.contains(&num) {
                ret = ret.min(num);
            }
        }
        for num in backs.iter() {
            if !same.contains(num) {
                ret = ret.min(*num);
            }
        }

        ret % 3000
    }
}

fn main() {}
