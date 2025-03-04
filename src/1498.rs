#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        // Sort the array to easily determine min and max of subsequences
        nums.sort();

        let modulo: i32 = 1_000_000_007;
        let n = nums.len();

        // Pre-compute powers of 2 modulo 10^9+7
        // powers[i] = 2^i % modulo
        let mut powers = vec![1; n];
        for i in 1..n {
            powers[i] = (powers[i - 1] * 2) % modulo;
        }

        let mut result = 0;
        let mut left: i32 = 0;
        let mut right: i32 = n as i32 - 1;

        while left <= right {
            if nums[left as usize] + nums[right as usize] > target {
                // If current sum is too large, move right pointer to find smaller max
                right -= 1;
            } else {
                // If nums[left] + nums[right] <= target, then all subsequences with
                // min = nums[left] and max <= nums[right] are valid

                // Number of elements between left and right (inclusive) = right - left + 1
                // Number of subsequences = 2^(right-left) (we choose any subset of elements between left+1 and right)
                result = (result + powers[right as usize - left as usize]) % modulo;

                // Move to next minimum
                left += 1;
            }
        }

        result
    }
}

fn main() {}
