struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut ret = 0;
        let mut cnt: HashMap<i32, i64> = HashMap::new();

        let mut pairs = 0;
        let mut i = 0;
        let sz = nums.len();
        let k = k as i64;
        for (j, &val) in nums.iter().enumerate() {
            pairs += *cnt.get(&val).unwrap_or(&0);
            *cnt.entry(val).or_default() += 1;
            while i < nums.len() && pairs >= k {
                ret += (sz - j) as i64;

                pairs -= cnt[&nums[i]] - 1;
                cnt.entry(nums[i]).and_modify(|v| *v -= 1);

                i += 1;
            }
        }

        ret
    }
}

fn main() {}
