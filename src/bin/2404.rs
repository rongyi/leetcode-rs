struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        for v in nums.into_iter() {
            if v % 2 != 0 {
                continue;
            }
            *cnt.entry(v).or_default() += 1;
        }
        let mut max_freq = 0;
        let mut max_key = -1;
        for (k, v) in cnt.into_iter().rev() {
            if v >= max_freq {
                max_key = k;
                max_freq = v;
            }
        }
        max_key
    }
}

fn main() {}
