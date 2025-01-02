#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut digits = vec![];

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.iter().product::<i32>() - digits.iter().sum::<i32>()
    }
}
fn main() {}
