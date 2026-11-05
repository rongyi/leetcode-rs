#![allow(dead_code)]


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for (i, &val) in nums.iter().enumerate() {
            *count.entry(val).or_insert(0) += 1;
            let &cur_freq = count.get(&val).unwrap();
            *freq.entry(cur_freq).or_insert(0) += 1;
            let cur_total = cur_freq * freq.get(&cur_freq).unwrap();
            // 1 1 2 2 4
            //       i
            if cur_total == (i + 1) as i32 && i < nums.len() - 1 {
                ret = ret.max(i + 2);
            } else if cur_total == i as i32 {
                // 1 1 1 2 2 4
                //         i
                ret = ret.max(i + 1);
            }
        }

        ret as i32
    }
}

fn main() {}
