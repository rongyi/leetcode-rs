#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut result = i32::MAX;
        let mut and_values = Vec::new();

        for &num in &arr {
            let mut new_and_values = vec![num];

            for &prev_and in &and_values {
                let new_and = prev_and & num;

                // Only add if it's a new value (optimization)
                if !new_and_values.contains(&new_and) {
                    new_and_values.push(new_and);
                }
            }

            for &value in &new_and_values {
                result = result.min((value - target).abs());
            }

            and_values = new_and_values;
        }

        result
    }
}

fn main() {}
