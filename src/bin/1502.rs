#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let n = arr.len();
        if n < 2 {
            return true;
        }
        let diff = arr[1] - arr[0];
        for i in 2..n {
            if arr[i] - arr[i - 1] != diff {
                return false;
            }
        }
        true
    }
}

fn main() {}
