#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max_dist = 0;
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                max_dist = max_dist.max(j as i32 - i as i32);
                j += 1;
            } else {
                i += 1;
                if j < i {
                    j = i;
                }
            }
        }

        max_dist
    }
}
fn main() {}
