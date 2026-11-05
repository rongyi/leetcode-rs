#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        // sliding window
        let sz = nums.len();
        let mut i = 0;
        let mut ret = 0;
        let mut zero_count = 0;

        for j in 0..sz {
            if nums[j] == 0 {
                zero_count += 1;
            }
            // shrink the window
            while zero_count > 1 {
                if nums[i] == 0 {
                    zero_count -= 1;
                }
                i += 1;
            }
            // this is a valid range
            let mut cur_win = j - i + 1;
            // we still need a delete action
            // if zero_count == 0 {
            //     cur_win -= 1;
            // } else {
            //     // the window has 1 zero, delete this will shrink the window by 1
            //     cur_win -= 1;
            // }
            cur_win -= 1;
            ret = ret.max(cur_win);
        }

        ret as _
    }
}
fn main() {}
