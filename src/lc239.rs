struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let sz = nums.len();
        let mut ret: Vec<i32> = Vec::new();
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        let k = k as usize;

        for i in 0..sz {
            if i >= k {
                let e = cnt.get_mut(&nums[i - k]).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&nums[i - k]);
                }
            }
            let e = cnt.entry(nums[i]).or_default();
            *e += 1;

            if i >= k - 1 {
                let (&key, _) = cnt.iter().rev().next().unwrap();
                ret.push(key);
            }
        }

        ret
    }
}

fn main() {}
