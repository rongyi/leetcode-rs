#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut xy = 0;
        let mut yx = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                if c1 == 'x' {
                    xy += 1;
                } else {
                    yx += 1;
                }
            }
        }
        if (xy + yx) % 2 != 0 {
            return -1;
        }
        (xy / 2 + yx / 2) + (xy % 2) * 2
    }
}

fn main() {}
