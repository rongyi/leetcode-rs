#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut odd_sum_count = 0; // Count of prefix sums that are odd
        let mut even_sum_count = 1; // Count of prefix sums that are even (including empty prefix)
        let mut current_sum = 0; // Running sum
        let mut result = 0; // Result counter

        for num in arr {
            current_sum += num;

            if current_sum % 2 == 0 {
                // Current prefix sum is even
                // All previous odd prefix sums will form odd subarrays with current position
                result = (result + odd_sum_count) % MOD;
                even_sum_count += 1;
            } else {
                // Current prefix sum is odd
                // All previous even prefix sums will form odd subarrays with current position
                result = (result + even_sum_count) % MOD;
                odd_sum_count += 1;
            }
        }

        result
    }
}

fn main() {}
