struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut ret = 0;

        for num in num_set.iter() {
            // this is the start number
            if !num_set.contains(&(num - 1)) {
                let mut cur = 1;
                let mut cur_num = *num;

                while num_set.contains(&(cur_num + 1)) {
                    cur_num += 1;
                    cur += 1;
                }

                ret = ret.max(cur);
            }
        }

        ret
    }
}

fn main() {}
