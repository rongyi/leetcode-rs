struct Solution;

use std::i32;
impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let sz = nums.len();
        let full_sum: i32 = nums.iter().copied().sum();
        let total_round = target / full_sum;
        let left_part = target % full_sum;
        if left_part == 0 {
            return total_round * sz as i32;
        }
        let mut win_sum = 0;
        let mut i = 0;
        let mut min_win = i32::MAX;
        for j in 0..sz * 2 {
            win_sum += nums[j % sz];
            while win_sum > left_part {
                win_sum -= nums[i % sz];
                i += 1;
            }
            if win_sum == left_part {
                min_win = min_win.min((j - i + 1) as i32);
            }
        }

        if min_win == i32::MAX {
            -1
        } else {
            total_round * sz as i32 + min_win
        }
    }
}
fn main() {}
