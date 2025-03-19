#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let n = n as usize;
        let mut nums = vec![0; n + 1];
        nums[1] = 1;

        let mut max_val = 1;

        for i in 2..=n {
            if i % 2 == 0 {
                nums[i] = nums[i / 2];
            } else {
                nums[i] = nums[i / 2] + nums[i / 2 + 1];
            }
            max_val = max_val.max(nums[i]);
        }

        max_val
    }
}
fn main() {}
