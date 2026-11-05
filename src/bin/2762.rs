struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let sz = nums.len();
        let mut ret: i64 = 0;
        let mut left = 0;

        let mut freq_map: BTreeMap<i32, i32> = BTreeMap::new();

        for right in 0..sz {
            *freq_map.entry(nums[right]).or_default() += 1;

            while !freq_map.is_empty()
                && freq_map.last_key_value().unwrap().0 - freq_map.first_key_value().unwrap().0 > 2
            {
                let cur = freq_map.get_mut(&nums[left]).unwrap();
                *cur -= 1;
                if *cur == 0 {
                    freq_map.remove(&nums[left]);
                }
                left += 1;
            }
            ret += (right - left + 1) as i64;
        }

        ret
    }
}

fn main() {}
