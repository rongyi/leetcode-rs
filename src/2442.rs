
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut uniq: HashSet<i32> = nums.into_iter().fold(HashSet::new(), |mut acc, cur| {
            acc.insert(cur);
            acc
        });

        for mut v in uniq.clone().into_iter() {
            let mut rv = 0;
            while v != 0 {
                rv *= 10;
                rv += v % 10;
                v /= 10;
            }
            uniq.insert(rv);
        }

        uniq.len() as _
    }
}

fn main() {}
