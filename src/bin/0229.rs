
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let threshold = n / 3;
        let mut freq = HashMap::new();
        let mut result = Vec::new();

        // Count frequencies
        for &num in nums.iter() {
            *freq.entry(num).or_insert(0) += 1;
        }

        // Find elements with count > n/3
        for (&num, &count) in freq.iter() {
            if count > threshold {
                result.push(num);
            }
        }

        result
    }
}

fn main() {}
