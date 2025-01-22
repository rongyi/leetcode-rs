#![allow(dead_code)]


struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut vals: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for (i, &num) in arr.iter().enumerate() {
            vals.entry(num).or_default().push(i);
        }
        let mut rank = 1;
        let mut ret = vec![0; arr.len()];
        for (_k, v) in vals.iter() {
            for &idx in v.iter() {
                ret[idx] = rank;
            }
            rank += 1;
        }
        ret
    }
}

fn main() {}
