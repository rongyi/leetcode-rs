
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, usize> = HashMap::new();
        for num in &nums {
            let e = cnt.entry(*num).or_insert(0);
            *e += 1;
            if *e > nums.len() / 2 {
                return *num;
            }
        }
        0
    }
}

fn main() {}
