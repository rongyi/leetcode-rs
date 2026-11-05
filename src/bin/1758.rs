#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        // case1. let's assume the string starts with '0'
        let mut acc_case1 = 0;
        let mut acc_case2 = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 1 {
                if c != '1' {
                    acc_case1 += 1;
                }
                if c != '0' {
                    acc_case2 += 1;
                }
            } else {
                if c != '0' {
                    acc_case1 += 1;
                }
                if c != '1' {
                    acc_case2 += 1;
                }
            }
        }
        // case2. let's assume the string starts with '1'
        acc_case1.min(acc_case2)
    }
}

fn main() {}
