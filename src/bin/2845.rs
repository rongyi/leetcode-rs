struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let m = modulo as i64;
        let k = k as i64;
        let mut count: HashMap<i64, i64> = HashMap::new();
        count.insert(0, 1);

        let mut cur = 0;
        let mut ret = 0;

        for num in nums.into_iter() {
            if num as i64 % m == k {
                cur += 1;
            }
            let rem = cur % m;
            let needed = (rem - k + m) % m;
            ret += *count.get(&needed).unwrap_or(&0);

            *count.entry(rem).or_default() += 1;
        }

        ret
    }
}

fn main() {}
