#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let modulo: i64 = 1_000_000_007;

        // Compute prefix sums for O(1) range sum queries
        let mut prefix_sums: Vec<i64> = vec![0; n + 1];
        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] + nums[i] as i64;
        }

        // Function to get sum of range [start, end] (inclusive)
        let range_sum =
            |start: usize, end: usize| -> i64 { prefix_sums[end + 1] - prefix_sums[start] };

        // Find next smaller element indices (right boundary)
        let mut right: Vec<usize> = vec![n; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while !stack.is_empty() && nums[*stack.last().unwrap()] > nums[i] {
                right[*stack.last().unwrap()] = i;
                stack.pop();
            }
            stack.push(i);
        }

        // Find previous smaller element indices (left boundary)
        let mut left: Vec<usize> = vec![n; n];
        stack.clear();

        for i in (0..n).rev() {
            while !stack.is_empty() && nums[*stack.last().unwrap()] >= nums[i] {
                left[*stack.last().unwrap()] = i;
                stack.pop();
            }
            stack.push(i);
        }

        // Calculate max min-product
        let mut max_product: i64 = 0;

        for i in 0..n {
            let left_bound = if left[i] == n { 0 } else { left[i] + 1 };
            let right_bound = if right[i] == n { n - 1 } else { right[i] - 1 };

            let sum = range_sum(left_bound, right_bound);
            let min_product = sum * nums[i] as i64;

            max_product = max_product.max(min_product);
        }

        // Return the answer modulo 10^9 + 7
        (max_product % modulo) as i32
    }
}

fn main() {}
