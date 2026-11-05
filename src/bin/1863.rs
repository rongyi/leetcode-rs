#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        // Approach 1: Using backtracking to generate all subsets and calculate XOR sum
        // Time: O(2^n), Space: O(n) for recursion stack

        fn backtrack(nums: &Vec<i32>, index: usize, current_xor: i32) -> i32 {
            // Base case: when we've processed all elements
            if index == nums.len() {
                return current_xor;
            }

            // Calculate sum for subsets that include the current element
            let with_current = backtrack(nums, index + 1, current_xor ^ nums[index]);

            // Calculate sum for subsets that exclude the current element
            let without_current = backtrack(nums, index + 1, current_xor);

            // Return the total sum
            with_current + without_current
        }

        // Start backtracking from index 0 with initial XOR value of 0
        backtrack(&nums, 0, 0)
    }
}

fn main() {}
