struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // 1. Sort the array
        nums.sort_unstable();

        let mut moves = 0;

        // 2. Iterate and enforce strict ordering: nums[i] > nums[i - 1]
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                let target = nums[i - 1] + 1;
                moves += target - nums[i];
                nums[i] = target;
            }
        }

        moves
    }
}

fn main() {}
