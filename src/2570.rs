
struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut group: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        for num in nums1.iter() {
            let (id, val) = (num[0], num[1]);
            group.entry(id).or_default().push(val);
        }
        for num in nums2.iter() {
            let (id, val) = (num[0], num[1]);
            group.entry(id).or_default().push(val);
        }

        group.iter().fold(Vec::new(), |mut acc, (k, v)| {
            acc.push(vec![*k, v.iter().sum()]);
            acc
        })
    }
}

fn main() {}
