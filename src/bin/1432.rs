#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut min_str = num.to_string();
        let mut max_str = num.to_string();

        for c in max_str.clone().chars() {
            if c != '9' {
                max_str = max_str.replace(c, "9");
                break;
            }
        }
        let first_char = min_str.chars().next().unwrap();
        if first_char != '1' {
            min_str = min_str.replace(first_char, "1");
        } else {
            for c in min_str.clone().chars() {
                if c != '0' && c != '1' {
                    min_str = min_str.replace(c, "0");
                    break;
                }
            }
        }
        let min_val: i32 = min_str.parse().unwrap();
        let max_val: i32 = max_str.parse().unwrap();

        max_val - min_val
    }
}
fn main() {}
