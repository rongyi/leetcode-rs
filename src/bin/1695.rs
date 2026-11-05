#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut max_sum = 0;
        let mut current_sum = 0;
        let mut set: HashSet<i32> = HashSet::new();
        let mut left = 0;

        for right in 0..nums.len() {
            let num = nums[right];

            while set.contains(&num) {
                set.remove(&nums[left]);
                current_sum -= nums[left];
                left += 1;
            }

            set.insert(num);
            current_sum += num;
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}

fn main() {}
