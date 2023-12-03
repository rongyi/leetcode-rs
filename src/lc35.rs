struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0 as i32;
        let mut r = (nums.len() - 1) as i32;
        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        l
    }
}
