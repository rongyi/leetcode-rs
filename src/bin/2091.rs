
struct Solution;

use std::i32;
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut max_val = i32::MIN;
        let mut min_val = i32::MAX;
        let mut min_idx = 0;
        let mut max_idx = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num < min_val {
                min_val = num;
                min_idx = i;
            }
            if num > max_val {
                max_val = num;
                max_idx = i;
            }
        }
        let l = min_idx.min(max_idx);
        let r = min_idx.max(max_idx);

        // delete from both side
        let sz = nums.len();
        let op1 = l + 1 + sz - r;
        // delete from front
        let op2 = r + 1;
        // delete from end;
        let op3 = sz - l;

        op1.min(op2).min(op3) as _
    }
}

fn main() {}
