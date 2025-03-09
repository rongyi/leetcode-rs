#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for window in arr.windows(3) {
            if window.iter().all(|&num| num % 2 == 1) {
                return true;
            }
        }

        false
    }
}
fn main() {}
