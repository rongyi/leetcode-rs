#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        arr1.into_iter().fold(0, |mut acc, cur| {
            if arr2.iter().all(|&i| (i - cur).abs() > d) {
                acc += 1;
            }
            acc
        })
    }
}

fn main() {}
