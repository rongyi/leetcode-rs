struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut pos_map: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            pos_map.entry(num).or_default().push(i);
        }
        let mut min_max_gap = sz;

        for idxs in pos_map.values() {
            let mut cur_gap = 0;
            for i in 0..idxs.len() - 1 {
                cur_gap = cur_gap.max(idxs[i + 1] - idxs[i]);
            }
            // and circular
            let circular_gap = sz - (idxs[idxs.len() - 1] - idxs[0]);
            cur_gap = cur_gap.max(circular_gap);

            min_max_gap = min_max_gap.min(cur_gap);
        }

        (min_max_gap / 2) as _
    }
}

fn main() {}
