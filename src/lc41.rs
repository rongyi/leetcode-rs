struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        // Step 1: Move all positive numbers to the correct position
        for i in 0..n {
            let mut num = nums[i];
            while num > 0 && num <= n as i32 && nums[num as usize - 1] != num {
                let temp = nums[num as usize - 1];
                nums[num as usize - 1] = num;
                num = temp;
            }
        }

        // Step 2: Find the first missing positive number
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        // If all positive numbers are present, the result is n + 1
        n as i32 + 1
    }
}
