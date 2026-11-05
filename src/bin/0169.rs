struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut major_num = nums[0];
        let mut cnt = 0;
        for &num in nums.iter() {
            if cnt == 0 {
                major_num = num;
                cnt = 1;
            } else if num == major_num {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        major_num
    }
}

fn main() {}
