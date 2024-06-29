#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sz = nums1.len();
        let mut swap = vec![sz as i32; sz];
        let mut no_swap = vec![sz as i32; sz];
        swap[0] = 1;
        no_swap[0] = 0;

        for i in 1..sz {
            // already ordered
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                swap[i] = swap[i - 1] + 1;
                no_swap[i] = no_swap[i - 1];
            }
            // can swap
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                swap[i] = swap[i].min(no_swap[i - 1] + 1);
                no_swap[i] = no_swap[i].min(swap[i - 1]);
            }
        }

        swap[sz - 1].min(no_swap[sz - 1])
    }
}

fn main() {}
