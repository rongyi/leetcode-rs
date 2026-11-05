#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let sz = arr.len();
        arr.sort();
        let median = arr[(sz - 1) / 2];
        arr.sort_by(|&l, &r| {
            let v1 = (l - median).abs();
            let v2 = (r - median).abs();

            v1.cmp(&v2).reverse().then(l.cmp(&r).reverse())
        });
        arr.truncate(k as usize);
        arr
    }
}

fn main() {}
