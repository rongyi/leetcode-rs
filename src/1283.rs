#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();
        while l < r {
            let mid = l + (r - l) / 2;
            let sum = nums.iter().map(|&x| (x + mid - 1) / mid).sum::<i32>();
            if sum > threshold {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        l
    }
}

fn main() {}
