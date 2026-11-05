struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ret = vec![];
        let mut win: BTreeMap<i32, i32> = BTreeMap::new();
        let k = k as usize;
        let sz = nums.len();
        for i in 0..k {
            *win.entry(nums[i]).or_default() += 1;
        }
        let (&first, _) = win.iter().rev().next().unwrap();
        ret.push(first);

        for i in k..sz {
            let pop_cnt = win.get_mut(&nums[i - k]).unwrap();
            *pop_cnt -= 1;
            if *pop_cnt == 0 {
                win.remove(&nums[i - k]);
            }

            *win.entry(nums[i]).or_default() += 1;

            let (&cur_max, _) = win.iter().rev().next().unwrap();
            ret.push(cur_max);
        }

        ret
    }
}

fn main() {}
