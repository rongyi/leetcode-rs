#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        // This function calculates the minimum number of operations needed to make all elements in the array equal
        // An operation is defined as reducing a value to the next smallest value in the array
        //
        // Approach:
        // 1. Sort the array to identify distinct values and their frequencies
        // 2. For each element, count how many operations are needed to reduce it to the minimum value
        // 3. Each time we encounter a new larger value, all subsequent elements will need one more operation
        //
        // Time Complexity: O(n log n) due to sorting
        // Space Complexity: O(1) as we only use a few variables
        let mut nums = nums;
        nums.sort_unstable();

        let mut operations = 0;
        let mut current_ops = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                current_ops += 1;
            }
            operations += current_ops;
        }

        operations
    }
}
fn main() {}
