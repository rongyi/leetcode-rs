#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut all_nums: HashSet<i32> = HashSet::new();
        for &num in arr.iter() {
            if all_nums.contains(&(num * 2)) || (num % 2 == 0 && all_nums.contains(&(num / 2))) {
                return true;
            }

            all_nums.insert(num);
        }
        false
    }
}

fn main() {}
