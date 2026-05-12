struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut found = false;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];
            if target == mid_val {
                found = true;
            }
            if target <= mid_val {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        let mut ret = vec![-1, -1];
        if found {
            ret[0] = right + 1;
        }
        left = 0;
        right = nums.len() as i32 - 1;
        found = false;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];
            if target == mid_val {
                found = true;
            }
            if target >= mid_val {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        if found {
            ret[1] = left - 1;
        }

        ret
    }
}

fn main() {}
