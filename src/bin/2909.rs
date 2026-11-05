struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // pre_min[i] stores the minimum value from index 0 to i
        let mut pre_min = vec![0; n];
        pre_min[0] = nums[0];

        for i in 1..n {
            pre_min[i] = pre_min[i - 1].min(nums[i]);
        }

        let mut min_sum = i32::MAX;
        let mut suffix_min = nums[n - 1];

        // Iterate backwards from the second-to-last element to the second element
        // This represents our 'j' (the peak)
        for j in (1..n - 1).rev() {
            // Check if nums[j] is a peak relative to the best values found so far
            if pre_min[j - 1] < nums[j] && suffix_min < nums[j] {
                min_sum = min_sum.min(pre_min[j - 1] + nums[j] + suffix_min);
            }
            // Update suffix_min for the next iteration (which moves left)
            suffix_min = suffix_min.min(nums[j]);
        }

        if min_sum == i32::MAX {
            -1
        } else {
            min_sum
        }
    }
}

fn main() {}
