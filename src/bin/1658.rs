#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let target = total_sum - x;

        // If target is 0, we need to remove all elements
        if target == 0 {
            return nums.len() as i32;
        }

        // If target is negative, it's impossible to achieve
        if target < 0 {
            return -1;
        }

        let mut current_sum = 0;
        let mut max_len = 0;
        let mut left = 0;

        // Sliding window approach
        for right in 0..nums.len() {
            current_sum += nums[right];

            // Shrink window until current_sum <= target
            while left <= right && current_sum > target {
                current_sum -= nums[left];
                left += 1;
            }

            // If we found a subarray with sum = target, update max_len
            if current_sum == target {
                max_len = max_len.max((right - left + 1) as i32);
            }
        }

        // If max_len is still 0, we couldn't find a valid subarray
        if max_len == 0 {
            return -1;
        }

        // Return the number of operations needed
        return nums.len() as i32 - max_len;
    }
}

fn main() {}
