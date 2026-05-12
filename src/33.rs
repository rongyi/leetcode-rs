struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];
            if mid_val == target {
                return mid;
            }
            if nums[left as usize] <= mid_val {
                // left half is sorted
                if target >= nums[left as usize] && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // right half is sorted
                if target > mid_val && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}

fn main() {}
