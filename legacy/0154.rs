struct Solution;

impl Solution {
    // contain duplicate
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] > nums[r] {
                l = mid + 1;
            } else if nums[mid] < nums[r] {
                r = mid;
            } else {
                r -= 1;
            }
        }

        nums[l]
    }
}

fn main() {}
