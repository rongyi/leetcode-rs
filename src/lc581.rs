struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz <= 1 {
            return 0;
        }
        let mut start = 0;
        let mut end = sz - 1;
        // jump with order
        while start < end && nums[start] <= nums[start + 1] {
            start += 1;
        }
        // already sorted
        if start == end {
            return 0;
        }
        while end > start && nums[end] >= nums[end - 1] {
            end -= 1;
        }
        let min_val = *nums[start..=end].iter().min().unwrap();
        let max_val = *nums[start..=end].iter().max().unwrap();
        while start > 0 && nums[start - 1] > min_val {
            start -= 1;
        }
        while end < sz - 1 && nums[end + 1] < max_val {
            end += 1;
        }

        (end - start + 1) as i32
    }
}

fn main() {}
