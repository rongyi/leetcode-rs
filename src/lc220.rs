struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut window: BTreeSet<i64> = BTreeSet::new();
        let k = index_diff as usize;
        let t = value_diff as i64;

        for (i, &num) in nums.iter().enumerate() {
            let num = num as i64;

            // yes, rust dont have lowerbound, but it has range
            if let Some(&val) = window.range((num - t)..=(num + t)).next() {
                if (num - val).abs() <= t {
                    return true;
                }
            }

            window.insert(num);

            if i >= k {
                window.remove(&(nums[i - k] as i64));
            }
        }

        false
    }
}

fn main() {}
