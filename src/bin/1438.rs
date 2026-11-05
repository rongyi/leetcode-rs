#![allow(dead_code)]

struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ret = 0;
        let mut s = BTreeSet::new();
        let mut j = 0;

        for i in 0..nums.len() {
            s.insert((nums[i], i));
            while let (Some(&(a, _)), Some(&(b, _))) = (s.first(), s.last()) {
                if b - a <= limit {
                    ret = ret.max(i - j + 1);
                    break;
                } else {
                    s.remove(&(nums[j], j));
                    j += 1;
                }
            }
        }

        ret as _
    }
}

fn main() {}
