#![allow(dead_code)]


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp: HashMap<i32, i32> = HashMap::new();
        let mut ret = 0;
        for &num in arr.iter() {
            let val = 1 + dp.get(&(num - difference)).unwrap_or(&0);
            dp.insert(num, val);
            ret = ret.max(val);
        }

        ret
    }
}

fn main() {}
