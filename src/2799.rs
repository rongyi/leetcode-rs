struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let total_distinct = nums.iter().collect::<HashSet<_>>().len();
        let mut ret = 0;
        let mut left = 0;
        let mut cur = HashMap::new();
        let sz = nums.len();

        for right in 0..sz {
            *cur.entry(nums[right]).or_insert(0) += 1;
            while cur.len() == total_distinct {
                ret += (sz - right) as i32;
                let left_val = nums[left];
                if let Some(freq) = cur.get_mut(&left_val) {
                    *freq -= 1;
                    if *freq == 0 {
                        cur.remove(&left_val);
                    }
                }
                left += 1;
            }
        }
        ret
    }
}

fn main() {}
