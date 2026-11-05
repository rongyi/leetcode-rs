struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[right] {
                // Minimum is in the right unsorted portion
                left = mid + 1;
            } else {
                // Minimum is at mid or in the left portion
                right = mid;
            }
        }

        nums[left]
    }
}

fn main() {}
