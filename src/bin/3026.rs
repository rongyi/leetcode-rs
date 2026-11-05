struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut min_prefix_map: HashMap<i32, i64> = HashMap::new();
        let mut current_prefix_sum: i64 = 0;
        let mut max_good_sum: i64 = i64::MIN;
        let mut found = false;

        for &x in &nums {
            // 1. Check if we can form a "good" subarray ending at x
            // Case A: starting element was x - k
            if let Some(&min_pre) = min_prefix_map.get(&(x - k)) {
                max_good_sum = max_good_sum.max(current_prefix_sum + x as i64 - min_pre);
                found = true;
            }
            // Case B: starting element was x + k
            if let Some(&min_pre) = min_prefix_map.get(&(x + k)) {
                max_good_sum = max_good_sum.max(current_prefix_sum + x as i64 - min_pre);
                found = true;
            }

            // 2. Update the map with the current element as a potential START
            // We store the prefix sum BEFORE adding x
            let entry = min_prefix_map.entry(x).or_insert(current_prefix_sum);
            if current_prefix_sum < *entry {
                *entry = current_prefix_sum;
            }

            // 3. Move prefix sum forward
            current_prefix_sum += x as i64;
        }

        if found {
            max_good_sum
        } else {
            0
        }
    }
}

fn main() {}
