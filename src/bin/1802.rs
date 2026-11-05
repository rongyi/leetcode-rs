#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let n = n as i64;
        let index = index as i64;
        let max_sum = max_sum as i64;

        // Binary search for the maximum possible value
        let mut left = 1;
        let mut right = max_sum;

        while left <= right {
            let mid = left + (right - left) / 2;

            if Self::get_sum(mid, n, index) <= max_sum {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right as i32
    }

    fn get_sum(value: i64, n: i64, index: i64) -> i64 {
        // Calculate sum on the left side (not including the index)
        let left_sum = Self::calculate_side_sum(value, index);

        // Calculate sum on the right side (not including the index)
        let right_sum = Self::calculate_side_sum(value, n - index - 1);

        // Return total sum (including the value at index)
        left_sum + value + right_sum
    }

    fn calculate_side_sum(value: i64, count: i64) -> i64 {
        // If value is larger than count, we have a sequence: value-1, value-2, ..., value-count
        if value > count {
            return value * count - count * (count + 1) / 2;
        }

        // Otherwise, we have: value-1, value-2, ..., 1, 1, ..., 1
        let ones = count - (value - 1);
        let arithmetic_sum = value * (value - 1) / 2;

        return ones + arithmetic_sum;
    }
}

fn main() {}
