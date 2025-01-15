#![allow(dead_code)]
struct Solution;


impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut l = 1;
        while l < n {
            if !l.to_string().contains('0') && !(n - l).to_string().contains('0') {
                return vec![l, n - l];
            }
            l += 1;
        }
        unreachable!()
    }
}

fn main() {}
