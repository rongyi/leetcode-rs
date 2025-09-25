struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1_is_odd = nums1.len() % 2 == 1;
        let nums2_is_odd = nums2.len() % 2 == 1;
        let v1 = nums1.into_iter().fold(0, |acc, cur| acc ^ cur);
        let v2 = nums2.into_iter().fold(0, |acc, cur| acc ^ cur);
        if nums1_is_odd && nums2_is_odd {
            return v1 ^ v2;
        }
        if nums1_is_odd {
            return v2;
        }
        if nums2_is_odd {
            return v1;
        }
        0
    }
}

fn main() {}
