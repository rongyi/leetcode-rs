
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut target: HashSet<i32> = (1..=k).collect();
        for (i, num) in nums.iter().rev().enumerate() {
            target.remove(num);
            if target.is_empty() {
                return i as i32 + 1;
            }
        }
        -1
    }
}

fn main() {}
