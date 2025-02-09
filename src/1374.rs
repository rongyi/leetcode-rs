#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 1 {
            "a".repeat(n as usize)
        } else {
            let mut ret = "a".repeat((n - 1) as usize);
            ret.push('b');
            ret
        }
    }
}

fn main() {}
