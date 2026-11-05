#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut increment_ops = 0;
        let mut double_ops = 0;

        for &num in &nums {
            if num == 0 {
                continue;
            }

            let mut n = num;
            let mut local_double_ops = 0;

            while n > 0 {
                if n % 2 == 1 {
                    // If odd, we need an increment operation
                    increment_ops += 1;
                    n -= 1;
                } else {
                    // If even and not 0, divide by 2 (corresponds to a double operation)
                    n /= 2;
                    local_double_ops += 1;
                }
            }

            // Update the maximum number of doubling operations needed
            double_ops = double_ops.max(local_double_ops);
        }

        // Total operations is sum of increment operations and maximum double operations
        increment_ops + double_ops
    }
}

fn main() {}
