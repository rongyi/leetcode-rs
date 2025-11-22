struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort_unstable();
        let mut l = 0;
        let mut r = nums[nums.len() - 1] - nums[0];
        while l < r {
            let mid = l + (r - l) / 2;
            if Self::can_form_pairs(&nums, mid, p) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
    fn can_form_pairs(nums: &[i32], mid: i32, p: i32) -> bool {
        let mut count = 0;
        let mut i = 0;
        while i < nums.len() - 1 && count < p {
            if nums[i + 1] - nums[i] <= mid {
                count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        count >= p
    }
}

fn main() {}
