#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // If we have 4 or fewer elements, we can make them all equal
        if n <= 4 {
            return 0;
        }

        // Sort the array to make it easier to analyze
        let mut sorted = nums.clone();
        sorted.sort();

        // Consider all possibilities of removing up to 3 elements
        // Option 1: Remove 3 largest elements
        let diff1 = sorted[n - 4] - sorted[0];

        // Option 2: Remove 2 largest and 1 smallest
        let diff2 = sorted[n - 3] - sorted[1];

        // Option 3: Remove 1 largest and 2 smallest
        let diff3 = sorted[n - 2] - sorted[2];

        // Option 4: Remove 3 smallest elements
        let diff4 = sorted[n - 1] - sorted[3];

        // Return the minimum difference among all options
        diff1.min(diff2).min(diff3).min(diff4)
    }
}

fn main() {}
