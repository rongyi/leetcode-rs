#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let sum: i64 = chalk.iter().map(|&x| x as i64).sum();
        let mut remaining = (k as i64) % sum;

        for (i, &c) in chalk.iter().enumerate() {
            if remaining < c as i64 {
                return i as i32;
            }
            remaining -= c as i64;
        }

        0
    }
}

fn main() {}
