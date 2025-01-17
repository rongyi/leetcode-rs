#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut num: Vec<u32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        for c in num.iter_mut() {
            if *c != 9 {
                *c = 9;
                break;
            }
        }
        num.into_iter().fold(0, |acc, cur| acc * 10 + cur) as i32
    }
}

fn main() {}
