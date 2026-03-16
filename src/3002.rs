struct Solution;
use std::cmp::min;
use std::collections::HashSet;

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let limit = n / 2;

        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();

        // Elements unique to set1
        let unique1 = set1.iter().filter(|x| !set2.contains(x)).count();
        // Elements unique to set2
        let unique2 = set2.iter().filter(|x| !set1.contains(x)).count();
        // Elements present in both
        let common = set1.len() - unique1;

        // How many unique elements can we contribute from each side?
        // We can't take more than n/2 from either side.
        let take1 = min(unique1, limit);
        let take2 = min(unique2, limit);

        // After taking unique elements, how much "room" is left in our n/2 quotas?
        let room1 = limit - take1;
        let room2 = limit - take2;

        // We can fill the remaining room with common elements.
        // The total unique elements will be:
        // (elements from set1) + (elements from set2) + (common elements we can fit)
        let total_possible = take1 + take2 + min(common, room1 + room2);

        total_possible as i32
    }
}

fn main() {}
