struct Solution;

use std::i32;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut min_sum = i32::MAX;
        let mut cur_sum = 0;
        for i in 1..sz - 1 {
            for j in i + 1..sz {
                cur_sum = nums[0] + nums[i] + nums[j];
                if cur_sum < min_sum {
                    min_sum = cur_sum;
                }
            }
        }
        min_sum
    }
}

fn main() {}
