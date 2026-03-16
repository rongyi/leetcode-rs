struct Solution;

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let n = nums1.len();
        let target = n / 2;

        let set1: HashSet<_> = nums1.into_iter().collect();
        let set2: HashSet<_> = nums2.into_iter().collect();

        let common: HashSet<_> = set1.intersection(&set2).cloned().collect();

        let only1: HashSet<_> = set1.difference(&common).cloned().collect();
        let only2: HashSet<_> = set2.difference(&common).cloned().collect();

        let take_only1 = only1.len().min(target);
        let take_only2 = only2.len().min(target);

        let remaining = target - take_only1;
        let take_common = common.len().min(remaining + (target - take_only2));

        (take_only1 + take_only2 + take_common) as i32
    }
}

fn main() {}
