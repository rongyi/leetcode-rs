#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut arr = vec![];

        arr.push(first);

        for i in 0..encoded.len() {
            arr.push(encoded[i] ^ arr[i]);
        }

        arr
    }
}

fn main() {}
