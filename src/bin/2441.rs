struct Solution;

use std::{collections::BTreeSet, ops::Neg};
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let nums: BTreeSet<i32> = nums.into_iter().fold(BTreeSet::new(), |mut acc, cur| {
            acc.insert(cur);
            acc
        });

        for &v in nums.iter().rev() {
            if v <= 0 {
                break;
            }
            if nums.contains(&v.neg()) {
                return v;
            }
        }
        -1
    }
}
fn main() {}
