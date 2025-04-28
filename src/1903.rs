#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let nums: Vec<char> = num.chars().collect();
        let sz = nums.len();

        for i in (0..sz).rev() {
            if nums[i].to_digit(10).unwrap() % 2 == 1 {
                return num[..=i].to_string();
            }
        }
        "".to_string()
    }
}

fn main() {}
