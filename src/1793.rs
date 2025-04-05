#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = k as usize;
        let mut right = k as usize;
        let n = nums.len();
        let mut min_val = nums[k as usize];
        let mut max_score = min_val;
        // The algorithm finds the maximum subarray score, which is defined as:
        // (minimum value in the subarray) * (length of the subarray)
        //
        // We start with a subarray containing only the element at index k.
        // Then we expand the subarray by choosing to include either the next element to the left or right,
        // based on which element has the larger value (to keep the minimum value as high as possible).
        //
        // After each expansion:
        // 1. We update the minimum value in the current subarray
        // 2. Calculate the new score: min_val * subarray_length
        // 3. Update the maximum score if the new score is higher
        //
        // We continue until we've included all elements of the array.

        while left > 0 || right < n - 1 {
            if left == 0 {
                right += 1;
                min_val = min_val.min(nums[right]);
            } else if right == n - 1 {
                left -= 1;
                min_val = min_val.min(nums[left]);
            } else if nums[left - 1] < nums[right + 1] {
                right += 1;
                min_val = min_val.min(nums[right]);
            } else {
                left -= 1;
                min_val = min_val.min(nums[left]);
            }

            max_score = max_score.max(min_val * (right - left + 1) as i32);
        }

        max_score
    }
}

fn main() {}
