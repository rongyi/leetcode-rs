struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut i = 1;
        while i < nums.len() && nums[i] == nums[i - 1] + 1 {
            sum += nums[i];
            i += 1;
        }

        let mut missing = sum;
        while nums.contains(&missing) {
            missing += 1;
        }

        missing
    }
}
fn main() {}
