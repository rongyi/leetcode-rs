#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
           let mut acc = 0;
        
        for i in 1..=n {
            if Self::valid(i) {
                acc += 1;
            }
        }
        
        acc
    }

    fn valid(n: i32) -> bool {
        let mut ret = false;
        let s = n.to_string();
        for c in s.chars() {
            if c == '3' || c == '4' || c == '7' {
                return false;
            }
            if c == '2' || c == '5' || c == '6' || c == '9' {
                ret = true;
            }
        }

        ret
    }
}

fn main() {}
