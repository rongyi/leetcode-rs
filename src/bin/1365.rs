#![allow(dead_code)]

struct Solution;

use std::collections::{BTreeMap, HashMap};
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut uniq: BTreeMap<i32, i32> = BTreeMap::new();
        let mut val_to_idxs: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, &val) in nums.iter().enumerate() {
            *uniq.entry(val).or_insert(0) += 1;
            val_to_idxs.entry(val).or_default().push(idx);
        }

        let mut ret: Vec<i32> = vec![0; nums.len()];
        let mut acc = 0;
        for (_i, (&val, &cnt)) in uniq.iter().enumerate() {
            for &idx in val_to_idxs.get(&val).unwrap().iter() {
                ret[idx] = acc;
            }
            acc += cnt;
        }

        ret
    }
}

fn main() {}
