#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|x, y| {
            let x1 = x.count_ones();
            let y1 = y.count_ones();
            if x1 == y1 {
                x.cmp(y)
            } else {
                x1.cmp(&y1)
            }
        });
        arr
    }
}

fn main() {}
