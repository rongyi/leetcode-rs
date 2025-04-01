#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let mut result = vec![0; n];
        let boxes: Vec<char> = boxes.chars().collect();

        // Count the total number of operations for the first position
        let mut ones_count = 0;
        let mut operations = 0;

        for i in 0..n {
            if boxes[i] == '1' {
                ones_count += 1;
                operations += i;
            }
        }

        result[0] = operations as i32;

        // Calculate operations for the rest of the positions
        let mut left_ones = 0;
        let mut right_ones = ones_count;

        for i in 1..n {
            if boxes[i - 1] == '1' {
                left_ones += 1;
                right_ones -= 1;
            }

            operations = operations - right_ones + left_ones;

            result[i] = operations as i32;
        }

        result
    }
}

fn main() {}
