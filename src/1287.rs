#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut cnt = 1;
        let t = arr.len() / 4;
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                cnt += 1;
                if cnt > t {
                    return arr[i];
                }
            } else {
                cnt = 1;
            }
        }
        arr[0]
    }
}

fn main() {}
