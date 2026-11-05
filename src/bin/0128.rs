
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.into_iter().collect();
        let mut lcs = 0;

        for &num in nums.iter() {
            // this is the begin, assume there is a chain
            if !nums.contains(&(num - 1)) {
                let mut cur = num;
                let mut cur_len = 0;
                while nums.contains(&cur) {
                    cur_len += 1;
                    cur += 1;
                }
                lcs = lcs.max(cur_len);
            }
        }

        lcs
    }
}

fn main() {}
