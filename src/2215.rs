
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cnt1: HashMap<i32, i32> = HashMap::new();
        let mut cnt2: HashMap<i32, i32> = HashMap::new();
        for &num in nums1.iter() {
            *cnt1.entry(num).or_default() += 1;
        }

        for &num in nums2.iter() {
            *cnt2.entry(num).or_default() += 1;
        }
        let uniq1: HashSet<i32> = cnt1
            .iter()
            .map(|(k, _v)| *k)
            .collect();
        let uniq2: HashSet<i32> = cnt2
            .iter()
            .map(|(k, _v)| *k)
            .collect();

        vec![
            uniq1
                .clone()
                .into_iter()
                .filter(|x| !uniq2.contains(&x))
                .collect(),
            uniq2
                .clone()
                .into_iter()
                .filter(|x| !uniq1.contains(&x))
                .collect(),
        ]
    }
}

fn main() {}
