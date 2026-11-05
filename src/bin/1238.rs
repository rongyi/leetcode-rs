#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut ret = vec![];
        for i in 0..(1 << n) {
            ret.push(start ^ i ^ (i >> 1));
        }

        ret
    }
}

fn main() {}
