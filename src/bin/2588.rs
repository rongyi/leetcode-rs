
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut ret = 0;
        let mut dp: HashMap<i32, i64> = HashMap::new();
        dp.insert(0, 1);
        let mut acc = 0;

        for &v in nums.iter() {
            acc ^= v;
            ret += *dp.get(&acc).unwrap_or(&0);
            *dp.entry(acc).or_default() += 1;
        }

        ret
    }
}

fn main() {}
