#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut delta = vec![0; 2 * limit as usize + 2];

        for i in 0..n / 2 {
            let a = nums[i];
            let b = nums[n - 1 - i];
            let sum = a + b;

            // Make all sums [2, 2*limit] at least 2 operations
            delta[2] += 2;
            delta[2 * limit as usize + 1] -= 2;

            // Making sum [min(a,b) + 1, max(a,b) + limit] takes 1 operation
            delta[std::cmp::min(a, b) as usize + 1] -= 1;
            delta[std::cmp::max(a, b) as usize + limit as usize + 1] += 1;

            // Making sum = a + b takes 0 operations
            delta[sum as usize] -= 1;
            delta[sum as usize + 1] += 1;
        }

        let mut result = n as i32;
        let mut curr = 0;

        for i in 2..=2 * limit as usize {
            curr += delta[i];
            result = result.min(curr);
        }

        result
    }
}

fn main() {}
