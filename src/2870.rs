
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for &num in nums.iter() {
            *freq.entry(num).or_default() += 1;
        }

        let mut ret = 0;
        for &v in freq.values().into_iter() {
            if v == 1 {
                return -1;
            }
            match v % 3 {
                0 => ret += v / 3,
                1 => ret += v / 3 + 1, // e.g. 7   3 + 3 + 1 ? no, 3 + 2 + 2 , v / 3 - 1 + 1 + 1
                2 => ret += v / 3 + 1, // e.g. 8   3 + 3 + 2   8 / 3 + 1
                _ => unreachable!(),
            }
        }
        ret
    }
}

fn main() {}
