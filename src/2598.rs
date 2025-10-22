struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for &v in nums.iter() {
            let r = (v % value + value) % value;
            *freq.entry(r).or_default() += 1;
        }
        let mut m = 0;
        loop {
            let r = m % value;
            if *freq.get(&r).unwrap_or(&0) == 0 {
                return m;
            }
            *freq.entry(r).or_default() -= 1;
            m += 1;
        }
        unreachable!()
    }
}

fn main() {}
