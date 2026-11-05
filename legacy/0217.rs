
struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut uniq = HashSet::new();
        for num in nums.into_iter() {
            if uniq.contains(&num) {
                return true;
            }
            uniq.insert(num);
        }
        false
    }
}

fn main() {}
