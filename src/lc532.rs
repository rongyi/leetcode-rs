
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for &num in nums.iter() {
            *freq.entry(num).or_insert(0) += 1;
        }
        for (&num, &cnt) in freq.iter() {
            if k == 0 {
                if cnt >= 2 {
                    ret += 1;
                }
            } else {
                if freq.contains_key(&(num + k)) {
                    ret += 1;
                }
            }
        }

        ret
    }
}

fn main() {}
