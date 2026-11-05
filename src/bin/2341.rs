struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &num in nums.iter() {
            *cnt.entry(num).or_default() += 1;
        }
        let mut ret = vec![0, 0];
        for &v in cnt.values() {
            ret[0] += v / 2;
            ret[1] += v % 2;
        }

        ret
    }
}

fn main() {}
