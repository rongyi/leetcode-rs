#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut idx = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == max_num {
                idx = i;
            } else if num * 2 > max_num {
                return -1;
            }
        }

        idx as i32
    }
}
fn main() {}
