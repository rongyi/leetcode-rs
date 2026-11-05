#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut groups: HashMap<i32, i32> = HashMap::new();

        fn sum_digit(val: i32) -> i32 {
            let mut sum = 0;
            let mut num = val;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            sum
        }

        for i in 1..=n {
            let sum = sum_digit(i);
            *groups.entry(sum).or_insert(0) += 1;
        }
        let mut max_count = 0;
        let mut count = 0;

        for &value in groups.values() {
            if value > max_count {
                max_count = value;
                count = 1;
            } else if value == max_count {
                count += 1;
            }
        }

        count
    }
}

fn main() {}
