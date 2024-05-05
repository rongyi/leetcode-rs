#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut ret = 0;

        let mut cur_max = 0;
        for i in 0..arr.len() {
            // The basic idea is to use max[] array to keep track of the max value until the current position,
            // and compare it to the sorted array (indexes from 0 to arr.length - 1). If the max[i] equals the
            // element at index i in the sorted array, then the final count++.
            cur_max = cur_max.max(arr[i]);
            // here i means in sorted array to here the max value should be i
            if i as i32 == cur_max {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
