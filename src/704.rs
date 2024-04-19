#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if target == nums[mid] {
                return mid as i32;
            } else if target > nums[mid] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        -1
    }
}

fn main() {}
