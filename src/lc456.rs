struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut s3 = i32::MIN;

        for i in (0..nums.len()).rev() {
            if nums[i] < s3 {
                return true;
            }
            while !stack.is_empty() && nums[i] > *stack.last().unwrap() {
                s3 = stack.pop().unwrap();
            }
            stack.push(nums[i]);
        }
        false
    }
}

fn main() {}
