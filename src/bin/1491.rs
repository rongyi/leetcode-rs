#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let sum: i32 = salary.iter().sum();
        let count: i32 = salary.len() as i32;
        (sum as f64 - *salary.iter().min().unwrap() as f64 - *salary.iter().max().unwrap() as f64)
            / (count - 2) as f64
    }
}

fn main() {}
