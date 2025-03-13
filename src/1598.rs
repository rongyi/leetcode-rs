#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;

        for log in logs {
            match log.as_str() {
                "../" => {
                    // Go to parent folder (if not already at main folder)
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                "./" => {
                    // Stay in same folder, do nothing
                }
                _ => {
                    // Go to child folder
                    depth += 1;
                }
            }
        }

        depth
    }
}
fn main() {}
