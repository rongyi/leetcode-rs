struct Solution;

use std::collections::{BTreeSet, HashSet};
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<i32>>();
        let mut max_streak = 0;

        for num in nums.iter() {
            let mut cur_streak = 0;
            // with overflow check
            let Some(mut currnet) = num.checked_mul(*num) else {
                continue;
            };

            while nums.contains(&currnet) {
                cur_streak += 1;
                let Some(c) = currnet.checked_mul(currnet) else {
                    break;
                };
                currnet = c;
            }
            max_streak = max_streak.max(cur_streak + 1);
        }

        if max_streak > 1 {
            max_streak
        } else {
            -1
        }
    }
}

fn main() {}
