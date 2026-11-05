struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total_sum = 0;

        // i is the start of the subarray
        for i in 0..n {
            let mut distinct_set = HashSet::new();

            // j is the end of the subarray
            for j in i..n {
                // Add the current element to the set
                distinct_set.insert(nums[j]);

                // The number of distinct elements is the size of the set
                let count = distinct_set.len() as i32;

                // Add the square of the count to our total
                total_sum += count * count;
            }
        }

        total_sum
    }
}

fn main() {}
