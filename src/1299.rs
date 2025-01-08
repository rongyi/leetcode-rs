#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let sz = arr.len();
        let mut max_val = arr[sz - 1];
        for i in (0..sz - 1).rev() {
            let tmp = arr[i];
            arr[i] = max_val;
            max_val = max_val.max(tmp);
        }
        arr[sz - 1] = -1;
        arr
    }
}

fn main() {}
