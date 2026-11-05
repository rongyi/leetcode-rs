#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut ret = 0;

        let mut m = 1;
        loop {
            let mx = n - m * (m - 1) / 2;
            if mx <= 0 {
                break;
            }
            
            if mx % m == 0 {
                ret += 1;
            }
            
            m += 1;
        }

        ret
    }
}

fn main() {}
