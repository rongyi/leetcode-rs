
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut merge1 = Vec::new();
        for &n1 in &nums1 {
            for &n2 in &nums2 {
                merge1.push(n1 + n2);
            }
        }
        let mut merge2: HashMap<i32, i32> = HashMap::new();
        for &n3 in &nums3 {
            for &n4 in &nums4 {
                *merge2.entry(n3 + n4).or_insert(0) += 1;
            }
        }
        let mut ret = 0;
        for &m1 in &merge1 {
            ret += *merge2.get(&-m1).unwrap_or(&0);
        }

        ret
    }
}

fn main() {}
