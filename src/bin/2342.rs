struct Solution;

use std::collections::{BTreeSet, HashMap};
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let dsum = |mut i: i32| -> i32 {
            let mut acc = 0;
            while i != 0 {
                acc += i % 10;
                i /= 10;
            }
            acc
        };
        // group by dsum
        let mut group: HashMap<i32, Vec<i32>> = HashMap::new();
        for &num in nums.iter() {
            group.entry(dsum(num)).or_default().push(num);
        }
        let mut ret = -1;

        for (_k, v) in group.iter_mut().filter(|x| x.1.len() > 1) {
            // pick the biggest two
            v.sort_unstable();
            let sum: i32 = v.iter().rev().take(2).sum();
            ret = ret.max(sum);
        }

        ret
    }
}

fn main() {}
