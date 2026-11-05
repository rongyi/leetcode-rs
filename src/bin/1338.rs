#![allow(dead_code)]
struct Solution;

use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for &num in arr.iter() {
            *freq.entry(num).or_insert(0) += 1;
        }
        // println!("{:?}", freq);
        let val_to_freq: BTreeMap<i32, Vec<i32>> =
            freq.into_iter().fold(BTreeMap::new(), |mut acc, cur| {
                acc.entry(cur.1).or_default().push(cur.0);
                acc
            });

        // println!("{:?}", val_to_freq);

        let mut total_collect = 0;
        let mut ret = 0;
        for (freq, vals) in val_to_freq.into_iter().rev() {
            let mut i = 0;
            while i < vals.len() && total_collect < sz as i32 / 2 {
                total_collect += freq;
                i += 1;
                ret += 1;
            }
            if total_collect >= sz as i32 / 2 {
                break;
            }
        }
        ret
    }
}

fn main() {
    Solution::min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
