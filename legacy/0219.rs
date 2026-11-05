
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(nums[i]);
            if i >= k as usize {
                set.remove(&nums[i - k as usize]);
            }
        }

        false
    }
}

fn main() {}
