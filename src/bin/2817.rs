struct Solution;

use std::{collections::BTreeSet, i32};
impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
        let mut vals: BTreeSet<i32> = BTreeSet::new();
        let x = x as usize;

        let mut min_diff = i32::MAX;
        for i in x..nums.len() {
            vals.insert(nums[i - x]);
            let cur = nums[i];
            if let Some(val) = vals.range(cur..).next() {
                min_diff = min_diff.min(*val - cur);
            }

            if let Some(val) = vals.range(..=cur).next_back() {
                min_diff = min_diff.min(cur - *val);
            }
        }

        min_diff
    }
}

fn main() {}
