#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let bytes = s.as_bytes();

        // Determine which characters to remove first
        let (first, second, first_value, second_value) = if x >= y {
            (b'a', b'b', x, y)
        } else {
            (b'b', b'a', y, x)
        };

        let mut stack = Vec::new();
        let mut gain = 0;

        // First pass: remove the higher value pair
        for &byte in bytes {
            if byte == second && !stack.is_empty() && *stack.last().unwrap() == first {
                stack.pop();
                gain += first_value;
            } else {
                stack.push(byte);
            }
        }

        // Second pass: remove the lower value pair
        let mut new_stack = Vec::new();
        for &byte in &stack {
            if byte == first && !new_stack.is_empty() && *new_stack.last().unwrap() == second {
                new_stack.pop();
                gain += second_value;
            } else {
                new_stack.push(byte);
            }
        }

        gain
    }
}

fn main() {}
