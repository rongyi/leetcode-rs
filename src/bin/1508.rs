#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let left = left as usize;
        let right = right as usize;
        let modulo = 1_000_000_007;

        // Calculate all subarray sums
        let mut sums = Vec::with_capacity(n * (n + 1) / 2);

        for start in 0..n {
            let mut current_sum = 0;
            for end in start..n {
                current_sum += nums[end];
                sums.push(current_sum);
            }
        }

        // Sort the sums
        sums.sort_unstable();

        // Calculate the sum of elements from left to right
        let mut result = 0;
        for i in left - 1..right {
            result = (result + sums[i]) % modulo;
        }

        result
    }
}

fn main() {}
