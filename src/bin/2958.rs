struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = std::collections::HashMap::new();
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..nums.len() {
            *freq.entry(nums[right]).or_insert(0) += 1;

            while *freq.get(&nums[right]).unwrap() > k {
                *freq.get_mut(&nums[left]).unwrap() -= 1;
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

fn main() {}
