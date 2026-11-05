
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut intersection: HashSet<i32> = nums[0].iter().copied().collect();
        for i in 1..nums.len() {
            let cur_nums: HashSet<i32> = nums[i].iter().copied().collect();

            intersection = cur_nums.intersection(&intersection).copied().collect();
        }
        let mut ret: Vec<i32> = intersection.iter().copied().collect();
        ret.sort();
        ret
    }
}

fn main() {}
