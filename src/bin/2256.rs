struct Solution;

use std::i64;
impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let sum: i64 = nums.iter().map(|x| *x as i64).sum();
        let mut acc = 0i64;
        let mut min_idx = 0;
        let mut min_diff = i64::MAX;
        for (i, &num) in nums.iter().enumerate() {
            acc += num as i64;
            if i == nums.len() - 1 {
                let val = sum / nums.len() as i64;
                if val < min_diff {
                    min_idx = i;
                    min_diff = val;
                }
            } else {
                let avg_first = acc / (i + 1) as i64;
                let avg_last = (sum - acc) / (nums.len() - i - 1) as i64;

                let val = (avg_first - avg_last).abs();
                if val < min_diff {
                    min_idx = i;
                    min_diff = val;
                }
            }
        }

        min_idx as i32
    }
}

fn main() {}
