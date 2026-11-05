#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        // rec2 top of rec1
        if rec2[1] >= rec1[3] {
            return false;
        }
        // right
        if rec2[0] >= rec1[2] {
            return false;
        }
        // down
        if rec1[1] >= rec2[3] {
            return false;
        }
        // left
        if rec2[2] <= rec1[0] {
            return false;
        }
        true
    }
}

fn main() {}
