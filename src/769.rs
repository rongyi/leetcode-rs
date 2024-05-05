#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut ret = 0;

        let mut cur_max = 0;
        for i in 0..arr.len() {
            cur_max = cur_max.max(arr[i]);
            if i as i32 == cur_max {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
