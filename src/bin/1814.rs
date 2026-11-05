#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        // According to the nice pair definition: nums[i] + rev(nums[j]) = nums[j] + rev(nums[i])
        // We can rearrange this to nums[i] - rev(nums[i]) = nums[j] - rev(nums[j])
        // So we count the occurrences of nums[i] - rev(nums[i]) for all i

        let modulo = 1_000_000_007;
        let mut count = HashMap::new();
        let mut result = 0;

        for num in nums {
            // Calculate num - rev(num)
            let rev = {
                let mut n = num;
                let mut rev = 0;
                while n > 0 {
                    rev = rev * 10 + n % 10;
                    n /= 10;
                }
                rev
            };

            let diff = num - rev;

            // Get current count for this diff
            let curr_count = *count.get(&diff).unwrap_or(&0);

            // Add the current count to result
            result = (result + curr_count) % modulo;

            // Increment the count for this diff
            count.insert(diff, curr_count + 1);
        }

        result
    }
}

fn main() {}
