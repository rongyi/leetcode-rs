#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let d1 = Self::find_clsest_divisor(num + 1);
        let d2 = Self::find_clsest_divisor(num + 2);
        if (d1[0] - d1[1]).abs() < (d2[0] - d2[1]).abs() {
            d1
        } else {
            d2
        }
    }

    fn find_clsest_divisor(num: i32) -> Vec<i32> {
        let sqrt = (num as f64).sqrt() as i32;
        for i in (1..=sqrt).rev() {
            if num % i == 0 {
                return vec![i, num / i];
            }
        }
        vec![1, num]
    }
}

fn main() {}
