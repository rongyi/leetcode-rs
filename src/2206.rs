
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &num in nums.iter() {
            *cnt.entry(num).or_default() += 1;
        }

        for (_k, v) in cnt.iter() {
            if *v % 2 != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {}
