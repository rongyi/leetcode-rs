#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min_diff = i32::MAX;
        let mut ret = Vec::new();
        for i in 1..arr.len() {
            let cur_diff = (arr[i] - arr[i - 1]).abs();
            if cur_diff < min_diff {
                ret.clear();
                min_diff = cur_diff;
            }
            if cur_diff == min_diff {
                ret.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ret
    }
}

fn main() {}
