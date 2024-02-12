
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnt1 = HashMap::new();
        let mut cnt2 = HashMap::new();
        for &num in &nums1 {
            *cnt1.entry(num).or_insert(0) += 1;
        }
        for num in nums2 {
            *cnt2.entry(num).or_insert(0) += 1;
        }
        let mut ret = Vec::new();
        for (num, &val1) in cnt1.iter() {
            if let Some(&val2) = cnt2.get(num) {
                ret.extend(vec![*num; val1.min(val2) as usize])
            }
        }
        ret
    }
}

fn main() {}
