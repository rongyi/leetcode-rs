struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // one half must be sorted
        let mut left = 0 as i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];
            if target == mid_val {
                return mid;
            }
            // check which side is sorted
            if nums[left as usize] <= mid_val {
                // left chunk is sorted
                if target >= nums[left as usize] && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // right chunk is sorted
                if target <= nums[right as usize] && target > mid_val {
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
