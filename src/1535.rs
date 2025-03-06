#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut max = arr[0];
        let mut count = 0;

        for i in 1..arr.len() {
            if arr[i] > max {
                max = arr[i];
                count = 1;
            } else {
                count += 1;
            }

            if count == k {
                return max;
            }
        }

        max
    }
}

fn main() {}
