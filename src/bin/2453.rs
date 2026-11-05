
struct Solution;

use std::{collections::HashMap, i32};
impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut max_cnt = 0;
        for &num in &nums {
            let k = num % space;
            *cnt.entry(k).or_default() += 1;
            if cnt[&k] > max_cnt {
                max_cnt = cnt[&k];
            }
        }
        let mut ret = i32::MAX;

        for &num in &nums {
            if cnt[&(num % space)] == max_cnt {
                ret = ret.min(num);
            }
        }

        ret
    }
}

fn main() {}
