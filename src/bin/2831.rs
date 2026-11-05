struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            pos.entry(num).or_default().push(i);
        }
        let mut max_len = 0;
        let k = k as usize;

        for p in pos.values() {
            let mut left = 0;
            for right in 0..p.len() {
                // let total_span = ;
                // let current_cnt = right - left + 1;
                while p[right] - p[left] - (right - left) > k {
                    left += 1;
                }
                max_len = max_len.max(right - left + 1);
            }
        }
        max_len as _
    }
}
fn main() {}
