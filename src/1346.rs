#![allow(dead_code)]


struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let zero_count: usize = arr.iter().filter(|&&x| x == 0).collect::<Vec<_>>().len();
        if zero_count >= 2 {
            return true;
        }
        // or just calculate
        let all_nums: HashSet<i32> = arr.iter().copied().collect();
        for &num in arr.iter() {
            if num != 0 && all_nums.contains(&(num * 2)) {
                return true;
            }
        }
        false
    }
}

fn main() {}
