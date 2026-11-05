struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut unique: Vec<i32> = nums
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique.sort_unstable();

        let m = unique.len();
        let mut left = 0;
        let mut max_window = 0;

        for right in 0..m {
            while unique[right] - unique[left] >= n {
                left += 1;
            }
            max_window = max_window.max(right - left + 1);
        }

        (n - max_window as i32) as i32
    }
}

fn main() {}
