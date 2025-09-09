struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut uniq: HashSet<i32> = nums.into_iter().collect();
        let mut cnt: Vec<i64> = vec![0; 30];

        for n in uniq.into_iter() {
            let idx = Self::digit_count(n) as usize;
            cnt[idx] += 1;
        }
        let mut ret = 0;

        for i in 1..cnt.len() {
            for j in 1..cnt.len() {
                if i + j >= k as usize {
                    ret += cnt[i] * cnt[j];
                }
            }
        }

        ret
    }
    fn digit_count(mut v: i32) -> i32 {
        let mut acc = 0;
        while v > 0 {
            acc += 1;
            v &= v - 1;
        }
        acc
    }
}
fn main() {}
