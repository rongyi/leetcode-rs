struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return -1;
        }

        // Step 1: Precompute the minimum value to the left of each index
        let mut left_min = vec![0; n];
        left_min[0] = nums[0];
        for i in 1..n {
            left_min[i] = left_min[i - 1].min(nums[i]);
        }

        // Step 2: Precompute the minimum value to the right of each index
        let mut right_min = vec![0; n];
        right_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            right_min[i] = right_min[i + 1].min(nums[i]);
        }

        let mut min_sum = i32::MAX;
        let mut found = false;

        // Step 3: Iterate through possible peaks (j)
        // A peak cannot be at the very start or very end
        for j in 1..n - 1 {
            if left_min[j - 1] < nums[j] && right_min[j + 1] < nums[j] {
                let current_sum = left_min[j - 1] + nums[j] + right_min[j + 1];
                min_sum = min_sum.min(current_sum);
                found = true;
            }
        }

        if found {
            min_sum
        } else {
            -1
        }
    }
}

fn main() {}
