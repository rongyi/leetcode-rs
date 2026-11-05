#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut max_time = 0;
        for &pos in &left {
            max_time = max_time.max(pos);
        }
        for &pos in &right {
            max_time = max_time.max(n - pos);
        }
        max_time
    }
}

fn main() {}
